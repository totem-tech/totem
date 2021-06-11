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

//! Centralises the types used in totem.

mod implementations;
pub mod traits;

use frame_support::{dispatch::EncodeLike, pallet_prelude::*};
use sp_std::prelude::*;

/// A collection based on `Vec` that guarantees that every member is unique.
#[derive(Decode, Encode)]
pub struct Set<T>(Vec<T>);

/// Balance on an account can be negative.
pub type LedgerBalance = i128;

/// General ledger account number.
pub type Account = u64;

/// The index number for identifying the posting to ledgers.
pub type PostingIndex = u128;

/// Used for comparisons.
pub type ComparisonAmounts = u128;

#[repr(u8)]
#[derive(Decode, Encode, Debug, Clone, Copy, PartialEq)]
pub enum RecordType {
    Teams,
    Timekeeping,
    Orders,
}

/// Note: Debit and Credit balances are account specific - see chart of accounts.
#[repr(u8)]
#[derive(Decode, Encode, Clone, Copy)]
pub enum Indicator {
    Debit = 0,
    Credit = 1,
}

pub mod accounting {
    use super::*;

    #[derive(Clone)]
    pub struct Record<AccountId, Hash, BlockNumber, Account, LedgerBalance> {
        pub primary_party: AccountId,
        pub counterparty: AccountId,
        pub ledger_account: Account,
        pub amount: LedgerBalance,
        pub debit_credit: Indicator,
        pub reference_hash: Hash,
        pub changed_on_blocknumber: BlockNumber,
        pub applicable_period_blocknumber: BlockNumber,
    }
}

pub mod funding {
    use super::*;

    // Not used
    #[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct TXKeysT<Hash> {
        pub tx_uid: Hash,
    }
}

pub mod orders {
    use super::*;

    // Module Types
    pub type OrderStatus = u16; // Generic Status for whatever the HashReference refers to

    #[repr(u8)]
    #[derive(Debug, Decode, Encode, Clone, Copy, PartialEq, Eq)]
    pub enum ApprovalStatus {
        Submitted = 0,
        Accepted = 1,
        Rejected = 2,
    }

    /// The order header: contains common values for all items.
    #[derive(PartialEq, Eq, Copy, Clone, Debug, Encode, Decode, Default)]
    pub struct OrderHeader<AccountId> {
        pub commander: AccountId,
        pub fulfiller: AccountId,
        pub approver: AccountId,
        pub order_status: u16,
        pub approval_status: ApprovalStatus,
        pub buy_or_sell: u16,
        pub amount: i128,
        pub market_order: bool,
        pub order_type: u16,
        pub deadline: u32,
        pub due_date: u32,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct OrderItem<Hash> {
        pub product: Hash,
        pub unit_price: i128,
        pub quantity: u128,
        pub unit_of_measure: u16,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct TXKeysL<Hash> {
        pub record_id: Hash,
        pub parent_id: Hash,
        pub bonsai_token: Hash,
        pub tx_uid: Hash,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct TXKeysM<Hash> {
        pub record_id: Hash,
        pub bonsai_token: Hash,
        pub tx_uid: Hash,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct TXKeysS<Hash> {
        pub bonsai_token: Hash,
        pub tx_uid: Hash,
    }
}

pub mod prefunding {
    use super::*;

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Decode, Encode, PartialEq, Eq)]
    pub enum LockStatus {
        Unlocked = 0,
        Locked = 1,
    }

    /// Generic Status for whatever the HashReference refers.
    //TODO
    pub type Status = u16;
}

pub mod teams {
    use super::*;

    /// Reference supplied externally.
    //TODO make an enum
    pub type ProjectStatus = u16;

    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
    pub struct DeletedProject<AccountId, ProjectStatus> {
        pub owned_by: AccountId,
        pub deleted_by: AccountId,
        pub status: ProjectStatus,
    }
}

pub mod timekeeping {
    use super::*;

    /// Number of pauses of the timer.
    pub type NumberOfBreaks = u16;

    /// Quantity of blocks determines the passage of time.
    pub type NumberOfBlocks = u64;

    pub type StartOrEndBlockNumber = NumberOfBlocks;

    /// submitted(0), accepted(1), rejected(2), disputed(3), blocked(4), invoiced(5), reason_code(0), reason text.

    /// Not calendar period, but fiscal periods 1-15 (0-14).
    pub type PostingPeriod = u16;

    //TODO: create an enum: Accepted (true)/Pending (false)
    pub type AcceptAssignedStatus = bool;

    /// Locked true, unlocked false.
    pub type LockStatus = bool;

    /// Reason for status change (TODO codes to be defined).
    pub type ReasonCode = u16;

    /// Category of reason code (TODO categories to be defined).
    pub type ReasonCodeType = u16;

    /// Reason for status change in text (not on chain!).
    // pub type ReasonCodeText = Vec<u8>;

    /// Ban status (default is false).
    pub type BanStatus = bool;

    #[derive(Clone, Copy, Debug, PartialEq, Encode, Decode)]
    pub enum StatusOfTimeRecord {
        Draft,
        Submitted,
        Disputed,
        Rejected,
        Accepted,
        Invoiced,
        Blocked,
    }

    /// Reason why the code changes.
    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct ReasonCodeStruct(pub ReasonCode, pub ReasonCodeType);

    /// Status of the code changes.
    #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default)]
    pub struct BannedStruct(BanStatus, ReasonCodeStruct);

    /// The individual time record.
    #[derive(PartialEq, Eq, Copy, Clone, Debug, Encode, Decode, Default)]
    pub struct Timekeeper<
        AccountId,
        ReferenceHash,
        NumberOfBlocks,
        LockStatus,
        StatusOfTimeRecord,
        ReasonCodeStruct,
        PostingPeriod,
        StartOrEndBlockNumber,
        NumberOfBreaks,
    > {
        pub worker: AccountId,
        pub project_hash: ReferenceHash,
        pub total_blocks: NumberOfBlocks,
        pub locked_status: LockStatus,
        pub locked_reason: ReasonCodeStruct,
        pub submit_status: StatusOfTimeRecord,
        pub reason_code: ReasonCodeStruct,
        pub posting_period: PostingPeriod,
        pub start_block: StartOrEndBlockNumber,
        pub end_block: StartOrEndBlockNumber,
        pub nr_of_breaks: NumberOfBreaks,
    }
}
