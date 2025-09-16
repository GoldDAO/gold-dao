use icrc_ledger_types::icrc21::errors::ErrorInfo;
use icrc_ledger_types::icrc21::errors::Icrc21Error;
use icrc_ledger_types::icrc21::requests::ConsentMessageMetadata;
use icrc_ledger_types::icrc21::requests::ConsentMessageRequest;
use icrc_ledger_types::icrc21::responses::ConsentInfo;
use icrc_ledger_types::icrc21::responses::ConsentMessage;

pub type Args = ConsentMessageRequest;
pub type Response = Result<ConsentInfo, Icrc21Error>;

// Creates a consent info message that combines:
// - The `intent` (always shown first)
// - The `generic_message`
// - The `fields` as "key: value" lines
pub fn create_consent_info(
    generic_message: String,
    intent: String,
    fields: Vec<(String, String)>,
    metadata: ConsentMessageMetadata,
) -> Response {
    // Build a single combined message string
    let mut combined_message = String::new();

    // Add intent first
    combined_message.push_str(&format!("Intent: {}\n", intent));

    // Add generic message (if not empty)
    if !generic_message.trim().is_empty() {
        combined_message.push_str(&format!("{}.\n", generic_message));
    }

    // Add all fields as "key: value" lines
    for (key, value) in fields {
        combined_message.push_str(&format!("{}: {}.\n", key, value));
    }

    let consent_message = ConsentMessage::GenericDisplayMessage(combined_message);

    Ok(ConsentInfo {
        consent_message,
        metadata,
    })
}

pub fn create_error_response(description: String) -> Response {
    let error_info = ErrorInfo { description };
    Err(Icrc21Error::UnsupportedCanisterCall(error_info))
}

pub fn handle_unsupported_method(args: Args) -> Response {
    create_error_response(format!("Method '{}' is not supported", args.method))
}
