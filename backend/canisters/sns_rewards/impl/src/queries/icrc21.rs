use candid::Decode;
use ic_cdk::query;
use icrc_ledger_types::icrc21::errors::ErrorInfo;
use icrc_ledger_types::icrc21::errors::Icrc21Error;
use icrc_ledger_types::icrc21::lib::MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES;
pub use sns_rewards_api_canister::icrc21::Args as Icrc21Args;
pub use sns_rewards_api_canister::icrc21::Response as Icrc21Response;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
pub use utils::icrcs::icrc21;
use utils::icrcs::icrc21::create_consent_info;
use utils::icrcs::icrc21::create_error_response;

#[derive(PartialEq, Debug, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Icrc21Function {
    ClaimRewards,
    ClaimRewardsBatch,
}

#[query]
pub fn icrc21_canister_call_consent_message(args: Icrc21Args) -> Icrc21Response {
    if args.arg.len() > MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES as usize {
        return Err(Icrc21Error::UnsupportedCanisterCall(ErrorInfo {
            description: format!(
                "The argument size is too large. The maximum allowed size is {} bytes.",
                MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES
            ),
        }));
    }

    match args.method.parse::<Icrc21Function>() {
        Ok(Icrc21Function::ClaimRewards) => handle_claim_rewards_consent(args),
        Ok(Icrc21Function::ClaimRewardsBatch) => handle_claim_rewards_batch_consent(args),
        Err(err) => create_error_response(format!("Unsupported method: {}", err)),
    }
}

fn handle_claim_rewards_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(&args.arg, sns_rewards_api_canister::claim_reward::Args) {
        Ok(claim_args) => {
            let fields = vec![
                ("Action".to_string(), "Claim Rewards".to_string()),
                ("Method".to_string(), args.method.clone()),
                ("Neuron ID".to_string(), claim_args.neuron_id.to_string()),
                ("Token".to_string(), claim_args.token.clone()),
            ];

            let generic_message = format!(
                "You are about to claim rewards for neuron ID `{}` using token `{}`",
                claim_args.neuron_id, claim_args.token
            );

            create_consent_info(
                generic_message,
                "Claim Rewards".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => create_error_response("Failed to decode ClaimRewards arguments.".to_string()),
    }
}

fn handle_claim_rewards_batch_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(
        &args.arg,
        sns_rewards_api_canister::claim_rewards_batch::Args
    ) {
        Ok(claim_args) => {
            let mut neuron_token_pairs = Vec::new();
            for arg in &claim_args.claim_reward_args {
                neuron_token_pairs.push(format!(
                    "Neuron ID: {}, Token: {}",
                    arg.neuron_id, arg.token
                ));
            }

            let fields = vec![
                ("Action".to_string(), "Claim Rewards Batch".to_string()),
                ("Method".to_string(), args.method.clone()),
                (
                    "Neurons & Tokens".to_string(),
                    neuron_token_pairs.join("; "),
                ),
            ];

            let generic_message = format!(
                "You are about to claim rewards for the following neurons and tokens: {}",
                neuron_token_pairs.join("; ")
            );

            create_consent_info(
                generic_message,
                "Claim Rewards Batch".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => {
            create_error_response("Failed to decode ClaimRewardsBatch arguments.".to_string())
        }
    }
}
