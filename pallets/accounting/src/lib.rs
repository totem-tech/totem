//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

//! The main Totem Global Accounting Ledger
//!
//! It handles all the ledger postings.
//! The account number follows the chart of accounts definitions and is constructed as a concatenation of:
//!
//! * Financial Statement Type Number int length 1 (Mainly Balance Sheet, Profit and Loss, and Memorandum)
//! * Account Category Number int length 1 (Mainly Assets, liabilities, Equity, Revenue and Expense, and non-balance sheet)
//! * Account Category Group number int length 1 (e.g. grouping expenses: operating expense, other opex, personnel costs)
//! * Accounting Group Nr concatenation of int length 4 + int length 4. The first four digits incrementing within the Category Group (e.g. range 1000-1999) for individual Accounting Group values
//! associated with the Category Group Number. The second four digits incrementing within the group (e.g. range 10001000-10001999) for individual Accounting Groups within the group itself.
//! * The last 4 ints are the Accounting Subgroup Number which specify where the value is posted.
//!
//! For example 250500120000011
//! Statement Type: Profit and Loss (2)
//! Account Category: Expenses (5)
//! Account Category Grp: Operating Expenses (0),
//! Accounting Group: Services (50012000),
//! Accounting Subgroup: Technical Assitance (0011)
//!
//! In other accounting systems there are further values hierarchically below the subgroup (for example if you have multiple bank accounts), but this is not necessary in Totem as this is
//! replaced by the notion of Identity. The key takeaway is that everything in Totem is a property of an Identity
//!
//! For example in reporting Amount_ou may drill down to the detail in a heirarchical report like this:
//! 110100010000000 Balance Sheet > Assets > Current Assets > Bank Current > CitiCorp Account (Identity)
//! 110100010000000 Balance Sheet > Assets > Current Assets > Bank Current > Bank of America Account (Identity)
//! Here the Ledger Account has a 1:n relationship to the identities, and therefore aggregates results
//!
//! In fact this is just the rearrangement of the attributes or properties of an individual identity
//! CitiCorp Account (Identity) has properties > Bank Current > Current Assets > Assets > Balance Sheet > 110100010000000
//! Bank of America Account (Identity) has properties > Bank Current > Current Assets > Assets > Balance Sheet > 110100010000000
//! Here the Identity has a 1:1 relationship to its properties defined in the account number that is being posted to
//!
//! # Totem Live Accounting Primitives
//!
//! * All entities operating on the Totem Live Accounting network have XTX as the Functional Currency. This cannot be changed.
//! * All accounting is carried out on Accrual basis.
//! * Accounting periods close every block, although entities are free to choose a specific block for longer periods (month/year close is a nominated block number, periods are defined by  block number ranges)
//! * In order to facilitate expense recognistion for example the period in which the transaction is recorded, may not necessrily be the period in which the
//! transaction is recognised) adjustments must specify the period(block number or block range) to which they relate. By default the transaction block number and the period block number are identical on first posting.
//!
//! # Curency Types
//!
//! The UI provides spot rate for live results for Period close reporting (also known as Reporting Currency or Presentation Currency), which is supported byt the exchange rates module.
//! General rules for Currency conversion at Period Close follow GAAP rules and are carried out as follows:
//! * Revenue recognition in the period when they occur, and expenses recognised (including asset consumption) in the same period as the revenue to which they relate
//! is recognised.
//! * All other expenses are recognised in the period in which they occur.
//! * Therefore the currency conversion for revenue and related expenses is calculated at the spot rate for the period (block) in which they are recognised.
//! * All other currency conversions are made at the rate for the period close. The UI can therefore present the correct conversions for any given value at any point in time.

#![cfg_attr(not(feature = "std"), no_std)]

//pub mod benchmarking;
//pub mod mock;
//pub mod tests;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{fail, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;

    use sp_runtime::traits::{Convert, Hash};
    use sp_std::{prelude::*, vec};

    use totem_common::{StorageMapExt, TryConvert};
    use totem_primitives::accounting::{
        Indicator::{self, *},
        Posting, Record,
    };
    use totem_primitives::{Account, LedgerBalance, PostingIndex};

    type CurrencyBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Every accounting post gets an index.
    #[pallet::storage]
    #[pallet::getter(fn posting_number)]
    pub type PostingNumber<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Associate the posting index with the identity.
    #[pallet::storage]
    #[pallet::getter(fn id_account_posting_id_list)]
    pub type IdAccountPostingIdList<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AccountId, Account), Vec<u128>>;

    /// Convenience list of Accounts used by an identity. Useful for UI read performance.
    #[pallet::storage]
    #[pallet::getter(fn accounts_by_id)]
    pub type AccountsById<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Account>>;

    /// Accounting Balances.
    #[pallet::storage]
    #[pallet::getter(fn balance_by_ledger)]
    pub type BalanceByLedger<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AccountId, Account), LedgerBalance>;

    /// Detail of the accounting posting (for Audit).
    #[pallet::storage]
    #[pallet::getter(fn posting_detail)]
    pub type PostingDetail<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (T::AccountId, Account, u128),
        (
            T::BlockNumber,
            LedgerBalance,
            Indicator,
            T::Hash,
            T::BlockNumber,
        ),
    >;

    /// Yay! Totem!
    #[pallet::storage]
    #[pallet::getter(fn global_ledger)]
    pub type GlobalLedger<T: Config> = StorageMap<_, Blake2_128Concat, Account, LedgerBalance>;

    /// Address to book the sales tax to and the tax jurisdiction (Experimental, may be deprecated in future).
    #[pallet::storage]
    #[pallet::getter(fn taxes_by_jurisdiction)]
    pub type TaxesByJurisdiction<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AccountId, T::AccountId), LedgerBalance>;

    // TODO
    // Quantities Accounting
    // Depreciation (calculated everytime there is a transaction so as not to overwork the runtime) - sets "last seen block" to calculate the delta for depreciation

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type AccountingConverter: TryConvert<CurrencyBalanceOf<Self>, LedgerBalance>
            + Convert<[u8; 32], Self::AccountId>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error fetching the balance by ledger.
        BalanceByLedgerFetching,
        /// Error fetching the global ledger.
        GlobalLedgerFetching,
        /// Posting index overflowed.
        PostingIndexOverflow,
        /// Balance Value overflowed.
        BalanceValueOverflow,
        /// Global Balance Value overflowed.
        GlobalBalanceValueOverflow,
        /// System failure in Account Posting.
        SystemFailure,
        /// Overflow error, amount too big.
        AmountOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0/*TODO*/)]
        pub fn opening_balance(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            todo!()
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn adjustment(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            todo!()
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        LegderUpdate(
            <T as frame_system::Config>::AccountId,
            Account,
            LedgerBalance,
            PostingIndex,
        ),
    }

    impl<T: Config> Pallet<T> {
        /// Basic posting function (warning! can cause imbalance if not called with corresponding debit or credit entries)
        /// The reason why this is a simple function is that (for example) one debit posting may correspond with one or many credit
        /// postings and vice-versa. For example a debit to Accounts Receivable is the gross invoice amount, which could correspond with
        /// a credit to liabilities for the sales tax amount and a credit to revenue for the net invoice amount. The sum of both credits being
        /// equal to the single debit in accounts receivable, but only one posting needs to be made to that account, and two posting for the others.
        /// The Totem Accounting Recipes are constructed using this simple function.
        /// The second Blocknumber is for re-targeting the entry in the accounts, i.e. for adjustments prior to or after the current period (generally accruals).
        fn post_amounts(
            key: Record<T::AccountId, T::Hash, T::BlockNumber, Account, LedgerBalance>,
            posting_index: u128,
        ) -> DispatchResultWithPostInfo {
            let ab: LedgerBalance = key.amount.abs();
            let balance_key = (key.primary_party.clone(), key.ledger_account);
            let posting_key = (key.primary_party.clone(), key.ledger_account, posting_index);
            let detail = (
                key.changed_on_blocknumber,
                ab,
                key.debit_credit,
                key.reference_hash,
                key.applicable_period_blocknumber,
            );
            // !! Warning !!
            // Values could feasibly overflow, with no visibility on other accounts. In this event this function returns an error.
            // Reversals must occur in the parent function (i.e. that calls this function).
            // As all values passed to this function are already signed +/- we only need to sum to the previous balance and check for overflow
            // Updates are only made to storage once tests below are passed for debits or credits.
            let new_balance = Self::balance_by_ledger(&balance_key)
                .ok_or(Error::<T>::BalanceByLedgerFetching)?
                .checked_add(key.amount)
                .ok_or(Error::<T>::BalanceValueOverflow)?;
            let new_global_balance = Self::global_ledger(&key.ledger_account)
                .ok_or(Error::<T>::GlobalLedgerFetching)?
                .checked_add(key.amount)
                .ok_or(Error::<T>::GlobalBalanceValueOverflow)?;

            PostingNumber::<T>::put(posting_index);
            IdAccountPostingIdList::<T>::mutate_or_err(&balance_key, |list| {
                list.push(posting_index)
            })?;
            //TODO Use a set?
            AccountsById::<T>::mutate_or_err(&key.primary_party, |accounts_by_id| {
                accounts_by_id.retain(|h| h != &key.ledger_account)
            })?;
            AccountsById::<T>::mutate_or_err(&key.primary_party, |accounts_by_id| {
                accounts_by_id.push(key.ledger_account)
            })?;
            BalanceByLedger::<T>::insert(&balance_key, new_balance);
            PostingDetail::<T>::insert(&posting_key, detail);
            GlobalLedger::<T>::insert(&key.ledger_account, new_global_balance);

            Self::deposit_event(Event::LegderUpdate(
                key.primary_party,
                key.ledger_account,
                key.amount,
                posting_index,
            ));

            Ok(().into())
        }
    }

    impl<T: Config> Posting<T::AccountId, T::Hash, T::BlockNumber, CurrencyBalanceOf<T>> for Pallet<T>
    where
        T: pallet_timestamp::Config,
    {
        type Account = Account;
        type LedgerBalance = LedgerBalance;
        type PostingIndex = PostingIndex;

        /// The Totem Accounting Recipes are constructed using this function which handles posting to multiple accounts.
        /// It is exposed to other modules as a trait
        /// If for whatever reason an error occurs during the storage processing which is sequential
        /// this function also handles reversing out the prior accounting entries
        /// Therefore the recipes that are passed as arguments need to be be accompanied with a reversal
        /// Obviously the last posting does not need a reversal for if it errors, then it was not posted in the first place.
        fn handle_multiposting_amounts(
            forward: Vec<Record<T::AccountId, T::Hash, T::BlockNumber, Account, LedgerBalance>>,
        ) -> DispatchResultWithPostInfo {
            let posting_index = Self::posting_number()
                .checked_add(1)
                .ok_or(Error::<T>::PostingIndexOverflow)?;
            let reverse = {
                let mut reverse = Vec::new();
                for key in forward.iter().cloned() {
                    reverse.push(Record {
                        amount: key.amount.checked_neg().ok_or(Error::<T>::AmountOverflow)?,
                        debit_credit: key.debit_credit.reverse(),
                        ..key
                    })
                }
                reverse
            };
            let mut tracking = Vec::with_capacity(forward.len());

            // Iterate over forward keys. If Ok add reversal key to tracking, if error, then reverse out prior postings.
            for (fwd, rev) in forward.into_iter().zip(reverse) {
                if let Err(e) = Self::post_amounts(fwd, posting_index) {
                    // Error before the value was updated. Need to reverse-out the earlier debit amount and account combination
                    // as this has already changed in storage.
                    for trk in tracking.into_iter() {
                        Self::post_amounts(trk, posting_index)
                            .or(Err(Error::<T>::SystemFailure))?;
                    }
                    fail!(e)
                }
                tracking.push(rev)
            }

            Ok(().into())
        }

        /// This function simply returns the Totem escrow account address
        fn get_escrow_account() -> T::AccountId {
            let escrow_account: [u8; 32] = *b"TotemsEscrowAddress4LockingFunds";

            T::AccountingConverter::convert(escrow_account)
        }

        /// This function takes the transaction fee and prepares to account for it in accounting.
        /// This is one of the few functions that will set the ledger accounts to be updated here. Fees
        /// are native to the Substrate Framework, and there may be other use cases.
        fn account_for_fees(
            fee: CurrencyBalanceOf<T>,
            payer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // Take the fee amount and convert for use with accounting. Fee is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let fee_converted: LedgerBalance =
                T::AccountingConverter::try_convert(fee).ok_or(Error::<T>::AmountOverflow)?;
            // Convert this for the inversion
            let inverted: LedgerBalance = -1 * fee_converted.clone();
            let increase_amount: LedgerBalance = fee_converted.into();
            let decrease_amount: LedgerBalance = inverted.into();
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing but allows posting to happen)
            let fee_hash: T::Hash = Self::get_pseudo_random_hash(payer.clone(), payer.clone());
            let keys = vec![
                Record::new(
                    payer.clone(),
                    payer.clone(),
                    250_50029000_0000_u64, // debit  increase 250500290000000 Totem Transaction Fees
                    increase_amount,
                    Credit,
                    fee_hash,
                    current_block,
                    current_block_dupe,
                ),
                Record::new(
                    payer.clone(),
                    payer.clone(),
                    110_10004000_0000_u64, // credit decrease 110100040000000 XTX Balance
                    decrease_amount,
                    Debit,
                    fee_hash,
                    current_block,
                    current_block_dupe,
                ),
            ];
            Self::handle_multiposting_amounts(keys)
        }

        fn get_pseudo_random_hash(sender: T::AccountId, recipient: T::AccountId) -> T::Hash {
            let input = (
                (sender, recipient),
                pallet_timestamp::Pallet::<T>::get(),
                sp_io::offchain::random_seed(),
                frame_system::Pallet::<T>::extrinsic_index(),
                frame_system::Pallet::<T>::block_number(),
            );

            T::Hashing::hash(input.encode().as_slice()) // default hash BlakeTwo256
        }
    }
}
