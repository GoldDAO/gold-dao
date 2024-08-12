use candid::{ Nat, Principal };
use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::TransferArg };
use sns_governance_canister::types::ListNeurons;
use sns_governance_canister::types::Neuron;
use tracing::debug;
use crate::state::read_state;
use tracing::{ error, info };
use utils::env::Environment;

pub async fn fetch_balance_of_sub_account(
    ledger_canister_id: Principal,
    sub_account: Subaccount
) -> Result<Nat, String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_balance_of(
            ledger_canister_id,
            &(Account {
                owner: read_state(|s| s.env.canister_id()),
                subaccount: Some(sub_account),
            })
        ).await
    {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("ERROR: {:?}", e.1)),
    }
}

use crate::state::SwapConfig;
use crate::token_swap::SwapClient;
use icpswap_client::ICPSwapClient;
pub fn build_icpswap_client(args: &SwapConfig, this_canister_id: Principal) -> Box<dyn SwapClient> {
    let input_token = args.input_token.clone();
    let output_token = args.output_token.clone();

    let (token0, token1) = if args.zero_for_one {
        (input_token, output_token)
    } else {
        (output_token, input_token)
    };

    Box::new(
        ICPSwapClient::new(
            this_canister_id,
            args.swap_canister_id,
            token0,
            token1,
            args.zero_for_one
        )
    )
}
