use crate::state::{mutate_state, read_state};
use canister_time::run_now_then_interval;
use canister_time::{run_interval, WEEK_IN_MS};
use canister_tracing_macros::trace;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::proposal::Action;
use sns_governance_canister::types::ExecuteGenericNervousSystemFunction;
use sns_governance_canister::types::ManageNeuron;
use sns_governance_canister::types::Proposal;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use types::CanisterId;
use types::Milliseconds;
use types::TokenInfo;

const PROPOSAL_CREATION_INTERVAL: Milliseconds = WEEK_IN_MS;
pub const BUYBACK_BURN_SUB_ACCOUNT: Subaccount = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const INTERVAL: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 1 week

pub fn start_job() {
    run_now_then_interval(INTERVAL, run);
}

pub fn run() {
    let canister_id = read_state(|state| state.data.proposal_config.sns_governance_canister);
    // NOTE: here we call the SNS function. In order to have the function on the SNS it should be registered using .
    let args = ExecuteGenericNervousSystemFunction {
        function_id: 0,
        payload: vec![0],
    };
    ic_cdk::spawn(create_proposal(canister_id, args));
}

// #[trace]
// TODO: change parameters
async fn create_proposal(canister_id: CanisterId, config: ExecuteGenericNervousSystemFunction) {
    let args = ManageNeuron {
        subaccount: Default::default(),
        command: Some(Command::MakeProposal(Proposal {
            title: "String".to_string(),
            summary: "String".to_string(),
            url: "String".to_string(),
            action: Some(Action::ExecuteGenericNervousSystemFunction(config)),
        })),
    };

    match sns_governance_canister_c2c_client::manage_neuron(canister_id, &args).await {
        // TODO: Handle the response somehow. Can I implement Debug here?
        Ok(_response) => {
            info!("Successfully created a proposal")
        }
        Err(e) => {
            error!("Failed to get SNS canisters summary: {:?}", e);
        }
    }
}

pub const SUB_ACCOUNT: Subaccount = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

async fn burn() {
    // TODO: Delete extra clone here
    let proposal_config = read_state(|s| (s.data.proposal_config.clone()));

    // Get the minting account in order to burn tokens (sending the to minting account is equivalent to burning)
    let minting_account = Account {
        owner: proposal_config.sns_governance_canister,
        subaccount: None,
    };

    // Specify GLDGov token details
    let gldgov_token_info = TokenInfo {
        ledger_id: proposal_config.gldgov_ledger_canister_id,
        fee: 100_000u64,
        decimals: 8u64,
    };

    // check the reserve pool has enough GLDGov to correctly transfer ( burn )
    match fetch_balance_of_sub_account(gldgov_token_info.ledger_id, SUB_ACCOUNT).await {
        Ok(balance) => {
            // TODO fix here in future. There should be no problem with converting u64 into u128.
            let total_to_burn = proposal_config.min_burn_amount + gldgov_token_info.fee as u128;
            if balance < total_to_burn {
                debug!(
                    "Balance of reserve pool : {} is too low to make a burn of {} plus a fee of {} ",
                    balance,
                    proposal_config.min_burn_amount,
                    gldgov_token_info.fee
                );
                return;
            }
        }
        Err(e) => {
            error!(e);
            return;
        }
    }

    // Send tokens to the minting account in order to burn them
    let args = TransferArg {
        from_subaccount: Some(BUYBACK_BURN_SUB_ACCOUNT),
        to: minting_account,
        fee: None,
        created_at_time: None,
        amount: proposal_config.min_burn_amount.into(),
        memo: None,
    };

    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        proposal_config.sns_governance_canister,
        &args,
    )
    .await
    {
        Ok(_) => {
            info!(
                "SUCCESS : {:?} GLDGov tokens burned from reserve pool",
                proposal_config.min_burn_amount
            );
            mutate_state(|s| {
                // s.data.last_daily_gldgov_burn = Some(current_time_ms);
            })
        }
        Err(e) => {
            error!(
                "ERROR : GLDGov failed to transfer from reserve pool to GLDGov minting account with error : {:?}",
                e
            );
        }
    }
}

use candid::Nat;
use candid::Principal;
use utils::env::Environment;
async fn fetch_balance_of_sub_account(
    ledger_canister_id: Principal,
    sub_account: Subaccount,
) -> Result<Nat, String> {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_canister_id,
        &(Account {
            owner: read_state(|s| s.env.canister_id()),
            subaccount: Some(sub_account),
        }),
    )
    .await
    {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("ERROR: {:?}", e.1)),
    }
}
