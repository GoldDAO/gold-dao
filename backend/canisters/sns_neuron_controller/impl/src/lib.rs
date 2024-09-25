/*!
# SNS neuron controller canister

The SNS neuron controller canister is responsible for managing
neurons. It automatically processes all the neurons: fetches updates,
claims rewards and re-distribute them to sns_rewards canister.

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
use queries::list_ogy_neurons::ListNeuronsResponse;

mod guards;
mod jobs;
mod lifecycle;
mod memory;
mod migrations;
mod queries;
mod state;
mod types;
mod updates;
mod utils;

use lifecycle::*;
use queries::*;
use updates::*;

export_candid!();
