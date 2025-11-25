use candid::Decode;
pub use gldt_swap_api_canister::icrc21::Args as Icrc21Args;
pub use gldt_swap_api_canister::icrc21::Response as Icrc21Response;
use ic_cdk::query;
use icrc_ledger_types::icrc21::errors::ErrorInfo;
use icrc_ledger_types::icrc21::errors::Icrc21Error;
use icrc_ledger_types::icrc21::lib::MAX_CONSENT_MESSAGE_ARG_SIZE_BYTES;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
pub use utils::icrcs::icrc21;
use utils::icrcs::icrc21::create_consent_info;
use utils::icrcs::icrc21::create_error_response;

#[derive(PartialEq, Debug, EnumString, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Icrc21Function {
    SwapNftForTokens,
    SwapTokensForNft,
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

    let message = match args.method.parse::<Icrc21Function>() {
        Ok(Icrc21Function::SwapNftForTokens) => handle_swap_nft_for_tokens_consent(args),
        Ok(Icrc21Function::SwapTokensForNft) => handle_swap_tokens_for_nft_consent(args),
        Err(err) => {
            return Err(Icrc21Error::UnsupportedCanisterCall(ErrorInfo {
                description: format!("Unsupported method: {}", err),
            }));
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

            for (i, nft) in manage_args.iter().enumerate() {
                fields.push((format!("NFT #{} ID", i + 1), nft.id.to_string()));
                fields.push((
                    format!("NFT Canister ID #{}", i + 1),
                    nft.canister_id.to_string(),
                ));
            }

            let generic_message = format!(
                "You are about to swap {} NFTs for tokens: {:?}.",
                nft_count, manage_args
            );

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
            // decoded_args is now Vec<Nft>
            // Let's create a summary string for multiple NFTs
            let nft_count = decoded_args.len();

            // Create a string listing all NFT IDs and their canister IDs
            let nft_list_str = decoded_args
                .iter()
                .map(|nft| {
                    format!(
                        "ID: {} from Canister: {}",
                        nft.id.0,
                        nft.canister_id.to_text()
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");

            // Fields describing the batch swap action
            let fields = vec![
                ("Action".to_string(), format!("Swap {} NFTs", nft_count)),
                ("Method".to_string(), args.method.clone()),
                ("NFTs".to_string(), nft_list_str.clone()),
            ];

            let generic_message = format!(
                "You are about to swap {} NFT(s): {}.",
                nft_count, nft_list_str
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
