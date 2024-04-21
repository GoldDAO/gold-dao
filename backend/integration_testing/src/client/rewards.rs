use crate::{ generate_pocket_query_call, generate_pocket_update_call };
use sns_governance_canister::types::NeuronId;
use sns_rewards::types::claim_neuron_response::UserClaimErrorResponse;
use types::NeuronInfo;
use sns_rewards::model::payment_processor::PaymentRound;
use types::{ HttpRequest, HttpResponse };
use sns_rewards::updates::set_reserve_transfer_amount::{
    SetReserveTransferAmountRequest,
    SetReserveTransferAmountResponse,
};
use sns_rewards::updates::set_reward_token_types::{
    SetRewardTokenTypesRequest,
    SetRewardTokenTypesResponse,
};
use types::TokenInfo;

use std::collections::HashMap;

use candid::Nat;
use types::TokenSymbol;

generate_pocket_query_call!(get_all_neurons);
generate_pocket_query_call!(get_neuron_by_id);
generate_pocket_update_call!(sync_neurons_manual_trigger);
generate_pocket_query_call!(get_active_payment_rounds);
generate_pocket_update_call!(sync_user_rewards);
generate_pocket_query_call!(http_request);
generate_pocket_update_call!(add_neuron_ownership);
generate_pocket_update_call!(remove_neuron_ownership);
generate_pocket_update_call!(claim_reward);
generate_pocket_update_call!(set_reserve_transfer_amounts);
generate_pocket_query_call!(set_reserve_transfer_amounts_validate);
generate_pocket_query_call!(get_reserve_transfer_amounts);
generate_pocket_query_call!(set_reward_token_types_validate);
generate_pocket_query_call!(get_reward_token_types);
generate_pocket_update_call!(set_reward_token_types);
// Updates
// generate_update_call!(icrc1_transfer);
pub mod claim_reward {
    use super::*;

    pub type Args = (NeuronId, String);
    pub type Response = Result<bool, UserClaimErrorResponse>;
}

pub mod add_neuron_ownership {
    use super::*;

    pub type Args = NeuronId;
    pub type Response = Result<NeuronId, UserClaimErrorResponse>;
}

pub mod remove_neuron_ownership {
    use super::*;

    pub type Args = NeuronId;
    pub type Response = Result<NeuronId, UserClaimErrorResponse>;
}

pub mod get_all_neurons {
    pub type Args = ();
    pub type Response = u64;
}

pub mod get_neuron_by_id {
    use super::*;

    pub type Args = NeuronId;
    pub type Response = Option<NeuronInfo>;
}

pub mod sync_neurons_manual_trigger {
    pub type Args = ();
    pub type Response = ();
}

pub mod get_active_payment_rounds {
    use super::*;
    pub type Args = ();
    pub type Response = Vec<PaymentRound>;
}

pub mod sync_user_rewards {
    pub type Args = ();
    pub type Response = ();
}

pub mod http_request {
    use super::*;

    pub type Args = HttpRequest;
    pub type Response = HttpResponse;
}

pub mod set_reserve_transfer_amounts {
    use super::*;

    pub type Args = SetReserveTransferAmountRequest;
    pub type Response = Result<SetReserveTransferAmountResponse, SetReserveTransferAmountResponse>;
}

pub mod set_reserve_transfer_amounts_validate {
    use super::*;

    pub type Args = SetReserveTransferAmountRequest;
    pub type Response = Result<String, String>;
}

pub mod get_reserve_transfer_amounts {
    use super::*;

    pub type Args = ();
    pub type Response = HashMap<TokenSymbol, Nat>;
}

pub mod set_reward_token_types {
    use super::*;

    pub type Args = SetRewardTokenTypesRequest;
    pub type Response = SetRewardTokenTypesResponse;
}

pub mod set_reward_token_types_validate {
    use super::*;

    pub type Args = SetRewardTokenTypesRequest;
    pub type Response = Result<String, String>;
}

pub mod get_reward_token_types {
    use super::*;

    pub type Args = ();
    pub type Response = HashMap<TokenSymbol, TokenInfo>;
}
