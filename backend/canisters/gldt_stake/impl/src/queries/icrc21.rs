use candid::Decode;
pub use gldt_stake_api_canister::icrc21::Args as Icrc21Args;
pub use gldt_stake_api_canister::icrc21::Response as Icrc21Response;
use gldt_stake_common::manage_stake_position_interface::ManageStakePositionArgs;
use ic_cdk::query;
use icrc_ledger_types::icrc21::lib::MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
pub use utils::icrcs::icrc21;
use utils::icrcs::icrc21::create_consent_info;
use utils::icrcs::icrc21::create_error_response;
use utils::icrcs::icrc21::icrc21_canister_call_consent_message::icrc21_consent_message_response;
use utils::icrcs::icrc21::icrc21_canister_call_consent_message::icrc21_error;
use utils::icrcs::icrc21::icrc21_canister_call_consent_message::icrc21_error_info;

#[derive(PartialEq, Debug, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Icrc21Function {
    GetPosition,
    ManageStakePosition,
}

#[query]
pub fn icrc21_canister_call_consent_message(args: Icrc21Args) -> Icrc21Response {
    if args.arg.len() > MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES as usize {
        return icrc21_consent_message_response::Err(icrc21_error::UnsupportedCanisterCall(
            icrc21_error_info {
                description: format!(
                    "The argument size is too large. The maximum allowed size is {} bytes.",
                    MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES
                ),
            },
        ));
    }

    let message = match args.method.parse::<Icrc21Function>() {
        Ok(Icrc21Function::GetPosition) => handle_get_position_consent(args),
        Ok(Icrc21Function::ManageStakePosition) => handle_manage_stake_position_consent(args),
        Err(err) => {
            return icrc21_consent_message_response::Err(icrc21_error::UnsupportedCanisterCall(
                icrc21_error_info {
                    description: format!("Unsupported method: {}", err),
                },
            ));
        }
    };

    message
}

fn handle_get_position_consent(args: Icrc21Args) -> Icrc21Response {
    create_consent_info(
        "You are fetching your stake position.".to_string(),
        "Get Position".to_string(),
        vec![
            ("Action".to_string(), "Get Stake Position".to_string()),
            ("Method".to_string(), args.method.clone()),
        ],
        args.user_preferences.metadata,
    )
}

fn handle_manage_stake_position_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(
        &args.arg,
        gldt_stake_api_canister::manage_stake_position::Args
    ) {
        Ok(manage_args) => {
            let mut fields = vec![
                ("Action".to_string(), "Manage Stake Position".to_string()),
                ("Method".to_string(), args.method.clone()),
            ];

            let generic_message = match manage_args {
                ManageStakePositionArgs::AddStake { amount } => {
                    fields.push(("Operation".to_string(), "Add Stake".to_string()));
                    fields.push(("Amount".to_string(), amount.to_string()));
                    format!("You are about to add {} to your stake position.", amount)
                }

                ManageStakePositionArgs::ClaimRewards { tokens } => {
                    fields.push(("Operation".to_string(), "Claim Rewards".to_string()));
                    fields.push((
                        "Tokens".to_string(),
                        tokens
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    ));
                    format!(
                        "You are about to claim rewards for the following tokens: {}.",
                        tokens
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }

                ManageStakePositionArgs::StartDissolving { fraction } => {
                    fields.push(("Operation".to_string(), "Start Dissolving".to_string()));
                    fields.push(("Fraction (%)".to_string(), fraction.to_string()));
                    format!("You are starting to dissolve {}% of your stake.", fraction)
                }

                ManageStakePositionArgs::DissolveInstantly { fraction } => {
                    fields.push(("Operation".to_string(), "Dissolve Instantly".to_string()));
                    fields.push(("Fraction (%)".to_string(), fraction.to_string()));
                    format!(
                        "You are instantly dissolving {}% of your stake. This may incur a penalty.",
                        fraction
                    )
                }

                ManageStakePositionArgs::Withdraw {} => {
                    fields.push(("Operation".to_string(), "Withdraw".to_string()));
                    "You are about to withdraw your fully dissolved stake.".to_string()
                }
            };

            create_consent_info(
                generic_message,
                "Stake Position Management".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => create_error_response(format!(
            "Failed to decode arguments for method '{}'",
            args.method
        )),
    }
}
