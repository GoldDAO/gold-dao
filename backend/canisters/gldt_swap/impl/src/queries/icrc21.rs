use candid::Decode;
pub use gldt_swap_api_canister::icrc21::Args as Icrc21Args;
pub use gldt_swap_api_canister::icrc21::Response as Icrc21Response;
use gldt_swap_common::swap::SwapId;
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
    SwapNftForTokens,
    SwapTokensForNft,
    RemoveIntentToSwap,
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
        Ok(Icrc21Function::SwapNftForTokens) => handle_swap_nft_for_tokens_consent(args),
        Ok(Icrc21Function::SwapTokensForNft) => handle_swap_tokens_for_nft_consent(args),
        Ok(Icrc21Function::RemoveIntentToSwap) => handle_remove_intent_to_swap_consent(args),
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

fn handle_swap_nft_for_tokens_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(&args.arg, gldt_swap_api_canister::swap_nft_for_tokens::Args) {
        Ok(manage_args) => {
            let mut fields = vec![
                ("Action".to_string(), "Swap NFT For Tokens".to_string()),
                ("Method".to_string(), args.method.clone()),
            ];

            let nft_count = manage_args.len();
            fields.push(("Number of NFTs".to_string(), nft_count.to_string()));

            for (i, (nft_id, principal)) in manage_args.iter().enumerate() {
                fields.push((format!("NFT #{} ID", i + 1), nft_id.0.to_string()));
                fields.push((format!("Recipient #{}", i + 1), principal.to_string()));
            }

            let generic_message = if nft_count == 1 {
                format!(
                    "You are about to swap 1 NFT (ID {}) for tokens to recipient {}.",
                    manage_args[0].0 .0, manage_args[0].1
                )
            } else {
                format!(
                    "You are about to swap {} NFTs for tokens to multiple recipients.",
                    nft_count
                )
            };

            create_consent_info(
                generic_message,
                "Swap NFT For Tokens".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => {
            create_error_response("Failed to decode swap NFT for tokens arguments".to_string())
        }
    }
}

pub fn handle_swap_tokens_for_nft_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(&args.arg, gldt_swap_api_canister::swap_tokens_for_nft::Args) {
        Ok(decoded_args) => {
            let fields = vec![
                ("Action".to_string(), "Swap Tokens For NFT".to_string()),
                ("Method".to_string(), args.method.clone()),
                ("NFT ID".to_string(), decoded_args.nft_id.0.to_string()),
                (
                    "NFT Canister ID".to_string(),
                    decoded_args.nft_canister_id.to_text(),
                ),
            ];

            let generic_message = format!(
                "You are about to swap tokens for NFT with ID {} from canister {}.",
                decoded_args.nft_id.0,
                decoded_args.nft_canister_id.to_text()
            );

            create_consent_info(
                generic_message,
                "NFT Swap".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => {
            create_error_response("Failed to decode swap tokens for NFT arguments".to_string())
        }
    }
}

pub fn handle_remove_intent_to_swap_consent(args: Icrc21Args) -> Icrc21Response {
    match Decode!(
        &args.arg,
        gldt_swap_api_canister::remove_intent_to_swap::Args
    ) {
        Ok(swap_id) => {
            let SwapId(nft_id, nat) = swap_id;
            let fields = vec![
                ("Action".to_string(), "Remove Intent To Swap".to_string()),
                ("Method".to_string(), args.method.clone()),
                ("NFT ID".to_string(), nft_id.0.to_string()),
                ("Swap Index".to_string(), nat.to_string()),
            ];

            let generic_message = format!(
                "You are about to remove the intent to swap for NFT ID {} with swap index {}.",
                nft_id.0, nat
            );

            create_consent_info(
                generic_message,
                "Remove Intent To Swap".to_string(),
                fields,
                args.user_preferences.metadata,
            )
        }
        Err(_) => {
            create_error_response("Failed to decode remove_intent_to_swap arguments".to_string())
        }
    }
}
