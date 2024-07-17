/*!
# Cycles manager canister

The Cycles Manager Canister solves the problem of overseeing and
managing the cycle balance in designated canisters. When a canister's
balance falls below a predefined threshold, the Cycles Manager Canister
automatically tops it up, thereby preventing service disruptions and
maintaining optimal performance.

## Copyright
© 2023  [DAO.LINK Sàrl], [Switzerland]

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

[DAO.LINK Sàrl]: https://daolink.org/about
[Switzerland]: https://www.zefix.ch/en/search/entity/list/firm/1264770
*/
use ic_cdk::export_candid;

use crate::state::take_state;
use crate::state::Data;
use crate::state::State;

mod guards;
mod jobs;
pub mod lifecycle;
mod memory;
mod model;
pub mod queries;
mod state;
pub mod updates;

use lifecycle::*;
use queries::*;
use updates::*;

export_candid!();

//test change
