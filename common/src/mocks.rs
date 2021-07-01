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

use crate::types::{
    accounting::Record, traits::accounting::Posting, traits::teams::Validating as TeamsValidating,
    traits::timekeeping::Validating as TimekeepingValidating,
};
use frame_support::dispatch::DispatchResultWithPostInfo;
use sp_std::vec::Vec;

impl<AccountId, Hash, BlockNumber, CoinAmount> Posting<AccountId, Hash, BlockNumber, CoinAmount>
    for ()
{
    type Account = ();
    type PostingIndex = u128;
    type LedgerBalance = i128;

    fn handle_multiposting_amounts(
        _fwd: Vec<Record<AccountId, Hash, BlockNumber, Self::Account, Self::LedgerBalance>>,
    ) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn account_for_fees(_f: CoinAmount, _p: AccountId) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn get_escrow_account() -> AccountId {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn get_pseudo_random_hash(_s: AccountId, _r: AccountId) -> Hash {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}

impl<AccountId, Hash> TeamsValidating<AccountId, Hash> for () {
    fn is_project_owner(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn is_project_valid(_h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn is_owner_and_project_valid(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}

impl<AccountId, Hash> TimekeepingValidating<AccountId, Hash> for () {
    fn is_time_record_owner(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn validate_and_archive(_o: AccountId, _h: Hash, _a: bool) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}
