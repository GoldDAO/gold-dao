/*!
# TODO: add canister description

## Copyright
© 2023  [Bochsler Assets & Securities (BAS) SA], [Switzerland]

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

[Bochsler Assets & Securities (BAS) SA]: https://bas.tech
[Switzerland]: https://www.zefix.ch/fr/search/entity/list/firm/1579921
*/
use cycles_manager_canister::init::InitArgs;
use cycles_manager_canister::update_config::Args;
use cycles_manager_canister::update_config::Response;
use ic_cdk::export_candid;

use crate::state::take_state;
use crate::state::Data;
use crate::state::State;
use types::{BuildVersion, Timestamped};

mod guards;
mod jobs;
mod lifecycle;
mod memory;
mod model;
mod queries;
mod state;
mod updates;
use std::cell::RefCell;

thread_local! {
    static WASM_VERSION: RefCell<Timestamped<BuildVersion>> = RefCell::default();
}

export_candid!();
