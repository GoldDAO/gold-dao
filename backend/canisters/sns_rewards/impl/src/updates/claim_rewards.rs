use candid::{ Nat, Principal };
use ic_cdk::update;
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::account::Account;
use sns_governance_canister::types::NeuronId;
use tracing::error;
use types::{ TokenInfo, TokenSymbol };

use utils::env::Environment;
use crate::{
    state::{ read_state, RuntimeState },
    types::claim_neuron_response::UserClaimErrorResponse,
    utils::{ authenticate_by_hotkey, fetch_neuron_data_by_id, transfer_token },
};

use UserClaimErrorResponse::*;

#[update]
async fn claim_reward(neuron_id: NeuronId, token: String) -> Result<bool, UserClaimErrorResponse> {
    let caller = read_state(|s| s.env.caller());
    claim_reward_impl(neuron_id, token, caller).await
}

pub async fn claim_reward_impl(
    neuron_id: NeuronId,
    token: String,
    caller: Principal
) -> Result<bool, UserClaimErrorResponse> {
    // verify the token symbol is valid
    let token_symbol = TokenSymbol::parse(&token).map_err(|err|
        TokenSymbolInvalid(
            format!("token of type {:?} is not a valid token symbol. error: {:?}", token, err)
        )
    )?;

    // get the token meta information associated with the valid token
    let token_info = read_state(|s: &RuntimeState|
        s.data.tokens.get(&token_symbol).copied()
    ).ok_or_else(||
        TokenSymbolInvalid(format!("Token info for type {:?} not found in state", token_symbol))
    )?;

    let neuron = fetch_neuron_data_by_id(&neuron_id).await?;

    // check the neuron contains the hotkey of the callers principal
    authenticate_by_hotkey(&neuron, &caller)?;
    let owner = read_state(|s| s.data.neuron_owners.get_owner_of_neuron_id(&neuron_id));
    match owner {
        Some(owner_principal) => {
            if owner_principal == caller {
                // neuron is owned by caller according to our state and has a valid hotkey
                return transfer_rewards(&neuron_id, owner_principal, &token_info).await;
            } else {
                return Err(NeuronOwnerInvalid(Some(owner_principal)));
            }
        }
        None => { Err(NeuronNotClaimed) }
    }
}

pub async fn transfer_rewards(
    neuron_id: &NeuronId,
    user_id: Principal,
    token_info: &TokenInfo
) -> Result<bool, UserClaimErrorResponse> {
    // get the balance of the sub account ( NeuronId is the sub account id )
    let balance_of_neuron_id = fetch_balance_of_neuron_id(token_info.ledger_id, neuron_id).await?;
    let amount_to_transfer = balance_of_neuron_id - token_info.fee;
    if amount_to_transfer == Nat::from(0u64) {
        return Err(TransferFailed("no rewards to claim".to_string()));
    }
    let neuron_sub_account: [u8; 32] = neuron_id.clone().into();
    let neuron_sub_account = Subaccount(neuron_sub_account);

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
        Err(e) => { Err(TransferFailed(e)) }
    }
}

async fn fetch_balance_of_neuron_id(
    ledger_canister_id: Principal,
    neuron_id: &NeuronId
) -> Result<Nat, UserClaimErrorResponse> {
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
        Err(e) => {
            error!("Fail - to neuron rewards: {:?}", e.1);
            Err(InternalError(e.1))
        }
    }
}
