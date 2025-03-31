use crate::state::read_state;
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use sns_governance_canister::types::manage_neuron::Command;
use sns_governance_canister::types::ManageNeuron;
use sns_governance_canister::types::{
    manage_neuron::DisburseMaturity, manage_neuron_response, ListNeurons, Neuron,
};
use tracing::debug;
use tracing::{error, info, trace};
use types::SnsNeuronId;

pub async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat,
) -> Result<(), String> {
    match icrc_ledger_canister_c2c_client::icrc1_transfer(
        ledger_id,
        &(TransferArg {
            from_subaccount: Some(from_sub_account),
            to: to_account,
            fee: None,
            created_at_time: None,
            amount,
            memo: None,
        }),
    )
    .await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

// Fetch all neurons from SNS governance canister
pub async fn fetch_neurons(
    sns_governance_canister_id: Principal,
    canister_id: Principal,
    is_test_mode: bool,
) -> Result<Vec<Neuron>, String> {
    let limit = 100;

    let mut args = ListNeurons {
        limit,
        start_page_at: None,
        of_principal: Some(canister_id),
    };

    let mut number_of_scanned_neurons = 0;
    let mut continue_scanning = true;

    let mut neurons = Vec::new();
    while continue_scanning {
        continue_scanning = false;
        debug!("Fetching neuron data");

        // NOTE: the reason why we need a loop here is that list_neurons can only return 100 neurons
        // at a time. In fact, I'm not sure that we would exceed the limit in any case, but it's
        // better to future proof it in case if it works that way.
        match sns_governance_canister_c2c_client::list_neurons(sns_governance_canister_id, &args)
            .await
        {
            Ok(response) => {
                let number_of_received_neurons = response.neurons.len();
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!(
                                "Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            if is_test_mode && number_of_scanned_neurons == 400 {
                                continue_scanning = false;
                            }
                            n.id.clone()
                        }
                    );
                }
                neurons.extend(response.neurons);
                number_of_scanned_neurons += number_of_received_neurons;
            }
            Err(e) => {
                error!("Failed to obtain all neurons data {:?}", e);
                return Err(format!("Failed to obtain all neurons data {:?}", e));
            }
        }
    }
    Ok(neurons)
}

pub enum RewardSumResult {
    Full(Nat),
    Partial(Nat, String),
    Empty,
}

impl RewardSumResult {
    pub fn get_internal(self) -> Nat {
        match self {
            RewardSumResult::Full(nat) => nat,
            RewardSumResult::Partial(nat, _) => nat,
            RewardSumResult::Empty => Nat::from(0u8),
        }
    }
}

pub enum ClaimRewardResult {
    Succesfull,
    Partial(String),
    Failed,
}

impl ClaimRewardResult {
    pub fn is_not_failed(&self) -> bool {
        !matches!(self, ClaimRewardResult::Failed)
    }
}

// TODO: think of outstanding payments struct in this context
pub async fn distribute_rewards(sns_ledger_canister_id: Principal) -> Result<(), String> {
    let sns_rewards_canister_id = read_state(|state| state.data.sns_rewards_canister_id);

    let fee = icrc_ledger_canister_c2c_client::icrc1_fee(sns_ledger_canister_id)
        .await
        .unwrap();
    // Transfer all the tokens to sns_rewards to be distributed
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        sns_ledger_canister_id,
        &(Account {
            owner: ic_cdk::api::id(),
            subaccount: None,
        }),
    )
    .await
    {
        Ok(balance) => {
            match transfer_token(
                [0; 32],
                sns_rewards_canister_id.into(),
                sns_ledger_canister_id,
                balance - fee,
            )
            .await
            {
                Ok(_) => {
                    info!("Successfully transferred rewards");

                    Ok(())
                }
                Err(error_message) => {
                    let error_message = format!("Error during transfer rewards: {}", error_message);
                    error!(error_message);
                    Err(error_message)
                }
            }
        }
        Err(e) => {
            let error_message = format!(
                "Failed to fetch token balance of sns_neuron_controller from ledger canister id {} with ERROR : {:?}",
                sns_ledger_canister_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

// NOTE: those tokens transaction is a minting transfer, from the governance canister's
// main account (which is also the minting account) to the provided account.
pub async fn disburse_neuron_maturity(
    sns_governance_canister_id: Principal,
    neuron_ids: Vec<SnsNeuronId>,
    to_account: Option<sns_governance_canister::types::Account>,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for neuron_id in neuron_ids {
        match sns_governance_canister_c2c_client::manage_neuron(
            sns_governance_canister_id,
            &ManageNeuron {
                subaccount: neuron_id.into(),
                command: Some(Command::DisburseMaturity(DisburseMaturity {
                    percentage_to_disburse: 100,
                    to_account: to_account.clone(),
                })),
            },
        )
        .await
        {
            Ok(manage_neuron_response) => match manage_neuron_response.command {
                Some(manage_neuron_response::Command::DisburseMaturity(response)) => {
                    trace!("Successfully disbursed maturity for neuron {:?}", response);
                }
                Some(response) => {
                    let error_msg =
                        format!("Unexpected response from manage_neuron: {:?}", response);
                    error!("{}", error_msg);
                    errors.push(error_msg);
                }
                None => {
                    let error_msg = "manage_neuron response contained no command.".to_string();
                    error!("{}", &error_msg);
                    errors.push(error_msg);
                }
            },
            Err(e) => {
                let error_msg = format!(
                    "Failed to disburse maturity for neuron {:?}: {:?}",
                    neuron_id, e
                );
                error!("{}", &error_msg);
                errors.push(error_msg);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// TODO: think on how to add delay here
use std::time::Duration;
pub async fn retry_with_attempts<F, Fut>(
    max_attempts: u8,
    _delay_duration: Duration,
    mut f: F,
) -> Result<(), String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                error!("Attempt {}: Error - {:?}", attempt, err);
                if attempt == max_attempts {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}
