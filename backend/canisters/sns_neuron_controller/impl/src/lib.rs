/*!
# SNS neuron controller canister

// TODO: add description

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
use lifecycle::init::InitArgs;
use queries::list_ogy_neurons::ListNeuronsResponse;
use updates::manage_recipients::{ManageRewardRecipientsRequest, ManageRewardRecipientsResponse};
use updates::manage_sns_neuron::{ManageSnsNeuronRequest, ManageSnsNeuronResponse};
use updates::stake_sns_neuron::StakeSnsNeuronResponse;

mod guards;
mod jobs;
mod lifecycle;
mod memory;
mod queries;
mod state;
mod testing;
mod types;
mod updates;
mod utils;

export_candid!();
