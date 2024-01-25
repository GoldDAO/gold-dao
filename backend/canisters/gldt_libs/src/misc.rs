use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

use crate::{ gldt_ledger, gld_nft::Account as GldNftAccount };

#[allow(dead_code)]
pub async fn dummy_await() {
    let gldt_ledger_canister_id = Principal::from_text("6uad6-fqaaa-aaaam-abovq-cai").unwrap_or(
        Principal::anonymous()
    );
    let service_ledger = gldt_ledger::Service(gldt_ledger_canister_id);
    log_message(
        format!("Sending dummy await - before time {}", ic_cdk::api::time() / 1_000_000_000)
    );
    let _ = service_ledger.icrc1_minting_account().await;
    log_message(
        format!("Received dummy await - after time {}", ic_cdk::api::time() / 1_000_000_000)
    );
}

// dummy for now until package compatibility is fixed
pub fn log_message(message: String) {
    ic_cdk::print(format!("GLDT: {}", message));
}

pub fn get_principal_from_gldnft_account(account: &GldNftAccount) -> Option<Principal> {
    match account {
        GldNftAccount::principal(p) => Some(*p),
        _ => None,
    }
}

pub fn convert_gld_nft_account_to_icrc1_account(account: GldNftAccount) -> Option<Account> {
    match account {
        GldNftAccount::principal(p) => Some(Account { owner: p, subaccount: None }),
        GldNftAccount::account { owner, sub_account } =>
            Some(Account { owner, subaccount: convert_bytebuf_to_array(sub_account) }),
        _ => None,
    }
}

fn convert_bytebuf_to_array(option_bytebuf: Option<serde_bytes::ByteBuf>) -> Option<[u8; 32]> {
    option_bytebuf.and_then(|buf| {
        let mut array = [0; 32];
        if buf.len() == 32 {
            array.copy_from_slice(&buf);
            Some(array)
        } else {
            None
        }
    })
}
