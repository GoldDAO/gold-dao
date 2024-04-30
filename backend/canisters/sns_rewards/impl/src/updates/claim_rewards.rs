use candid::{ Nat, Principal };
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use sns_governance_canister::types::NeuronId;
pub use sns_rewards_api_canister::claim_reward::{
    Args as ClaimRewardArgs,
    Response as ClaimRewardResponse,
};
use tracing::error;
use types::{ TokenInfo, TokenSymbol };

use utils::env::Environment;
use crate::{
    state::{ read_state, RuntimeState },
    utils::{
        authenticate_by_hotkey,
        fetch_neuron_data_by_id,
        transfer_token,
        AuthenticateByHotkeyResponse,
        FetchNeuronDataByIdResponse,
    },
};

#[update]
async fn claim_reward(args: ClaimRewardArgs) -> ClaimRewardResponse {
    let caller = read_state(|s| s.env.caller());
    claim_reward_impl(args.neuron_id, args.token, caller).await
}

pub async fn claim_reward_impl(
    neuron_id: NeuronId,
    token: String,
    caller: Principal
) -> ClaimRewardResponse {
    // verify the token symbol is valid
    let token_symbol = match TokenSymbol::parse(&token) {
        Ok(token) => token,
        Err(e) => {
            return ClaimRewardResponse::TokenSymbolInvalid(
                format!("{e} : token of type {token:?} is not a valid token symbol.")
            );
        }
    };

    // get the token meta information associated with the valid token
    let token_info = match read_state(|s: &RuntimeState| s.data.tokens.get(&token_symbol).copied()) {
        Some(token) => token,
        None => {
            return ClaimRewardResponse::TokenSymbolInvalid(
                format!("Token info for type {token_symbol:?} not found in state")
            );
        }
    };

    let neuron = fetch_neuron_data_by_id(&neuron_id).await;
    let neuron = match neuron {
        FetchNeuronDataByIdResponse::InternalError(e) => {
            return ClaimRewardResponse::InternalError(e);
        }
        FetchNeuronDataByIdResponse::NeuronDoesNotExist => {
            return ClaimRewardResponse::NeuronDoesNotExist;
        }
        FetchNeuronDataByIdResponse::Ok(n) => n,
    };

    // check the neuron contains the hotkey of the callers principal
    match authenticate_by_hotkey(&neuron, &caller) {
        AuthenticateByHotkeyResponse::NeuronHotKeyAbsent => {
            return ClaimRewardResponse::NeuronHotKeyAbsent;
        }
        AuthenticateByHotkeyResponse::NeuronHotKeyInvalid => {
            return ClaimRewardResponse::NeuronHotKeyInvalid;
        }
        AuthenticateByHotkeyResponse::Ok(_) => {}
    }
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(owner_principal) => {
            if owner_principal == caller {
                // neuron is owned by caller according to our state and has a valid hotkey
                match transfer_rewards(&neuron_id, owner_principal, &token_info).await {
                    Ok(amount) => ClaimRewardResponse::Ok(amount),
                    Err(e) => ClaimRewardResponse::TransferFailed(e),
                }
            } else {
                return ClaimRewardResponse::NeuronOwnerInvalid(Some(owner_principal));
            }
        }
        None => { ClaimRewardResponse::NeuronNotClaimed }
    }
}

pub async fn transfer_rewards(
    neuron_id: &NeuronId,
    user_id: Principal,
    token_info: &TokenInfo
) -> Result<bool, String> {
    // get the balance of the sub account ( NeuronId is the sub account id )
    let balance_of_neuron_id = fetch_balance_of_neuron_id(token_info.ledger_id, neuron_id).await?;
    if balance_of_neuron_id <= Nat::from(token_info.fee) {
        return Err(
            format!(
                "Your balance must be higher than the transfer fee of {}",
                Nat::from(token_info.fee)
            )
        );
    }
    let amount_to_transfer = balance_of_neuron_id - Nat::from(token_info.fee);
    if amount_to_transfer == Nat::from(0u64) {
        return Err("no rewards to claim".to_string());
    }
    let neuron_sub_account: Subaccount = neuron_id.clone().into();

    let user_account = Account {
        owner: user_id,
        subaccount: None,
    };
    // transfer the tokens to the claimer
    let transfer = transfer_token(
        neuron_sub_account,
        user_account,
        token_info.ledger_id,
        amount_to_transfer
    ).await;

    match transfer {
        Ok(_) => { Ok(true) }
        Err(e) => { Err(e) }
    }
}

async fn fetch_balance_of_neuron_id(
    ledger_canister_id: Principal,
    neuron_id: &NeuronId
) -> Result<Nat, String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(neuron_id.into()),
            })
        ).await
    {
        Ok(t) => { Ok(t) }
        Err((rejection_code, message)) => {
            error!("Fail - to neuron rewards: {:?}", message);
            Err(format!("{rejection_code:?} : {message}"))
        }
    }
}
