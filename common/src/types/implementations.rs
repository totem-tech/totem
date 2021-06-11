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

use super::{accounting::*, orders::*, prefunding::*, *};

impl<T> Default for Set<T> {
    fn default() -> Self {
        Set(Default::default())
    }
}

impl<T> Set<T>
where
    T: Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, elem: T) {
        self.remove(&elem);
        self.0.push(elem)
    }

    pub fn remove(&mut self, elem: &T) {
        self.0.retain(|x| x != elem)
    }
}

impl EncodeLike<Indicator> for bool {}

impl Indicator {
    pub fn reverse(self) -> Self {
        match self {
            Self::Debit => Self::Credit,
            Self::Credit => Self::Debit,
        }
    }
}

impl EncodeLike<RecordType> for u8 {}

impl<AccountId, Hash, BlockNumber, Account, LedgerBalance>
    Record<AccountId, Hash, BlockNumber, Account, LedgerBalance>
{
    pub fn new(
        primary_party: AccountId,
        counterparty: AccountId,
        ledger_account: Account,
        amount: LedgerBalance,
        debit_credit: Indicator,
        reference_hash: Hash,
        changed_on_blocknumber: BlockNumber,
        applicable_period_blocknumber: BlockNumber,
    ) -> Self {
        Record {
            primary_party,
            counterparty,
            ledger_account,
            amount,
            debit_credit,
            reference_hash,
            changed_on_blocknumber,
            applicable_period_blocknumber,
        }
    }
}

impl EncodeLike<ApprovalStatus> for u8 {}

impl Default for ApprovalStatus {
    fn default() -> Self {
        ApprovalStatus::Submitted
    }
}

impl EncodeLike<LockStatus> for bool {}
