use crate::{
    model::reward_pool::RewardPool,
    state::{mutate_state, read_state},
};
use ic_cdk::api::call;
use ic_cdk::update;
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
};
use sha256::sha256;
use types::{TokenType, TransferTokenArgs};
use utils::consts::OGY_LEDGER_CANISTER_ID;

#[update]
pub async fn transfer_funds_to_reward_pool(args: TransferTokenArgs) -> Result<BlockIndex, String> {
    match args.token.token_type {
        ICP => transfer_icp(args),
        OGY => transfer_ogy(args)
    }
}

async fn transfer_icp(args: TransferTokenArgs) {

    let this_canister_id = ic_cdk::api::id();

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(args.amount),
        fee: Tokens::from_e8s(10_000),
        from_subaccount: None,
        to: AccountIdentifier::new(&this_canister_id, &DEFAULT_SUBACCOUNT),
        created_at_time: None,
    };

    ic_ledger_types::transfer(args.from_canister, transfer_args)
        .await
        .map_err(|e| format!("sns_rewards - transffer_funds_to_reward_pool - failed to call ICP ledger - : {:?}", e))?
        .map_err(|e| format!("sns_rewards - transffer_funds_to_reward_pool - failed to transfer ICP to sns_rewards - : {:?}", e))
}

async fn transfer_ogy(args: TransferTokenArgs) {

    let this_canister_id = ic_cdk::api::id();

    let transfer_args = icrc_ledger_types::icrc1::transfer::TransferArg {
        from_subaccount: None,
        to: AccountIdentifier::new(&this_canister_id, &DEFAULT_SUBACCOUNT),
        fee: None,
        memo: Memo(0),
        amount: Tokens::from_e8s(args.amount),
        created_at_time : None
    };

    call(
        OGY_LEDGER_CANISTER_ID, // Replace with the actual ICRC-1 token canister ID
        "icrc1_transfer",
        (transfer_args, args.amount),
    )
    .await
    .map_err(|e| format!("sns_rewards - transffer_funds_to_reward_pool - failed to transfer OGY to sns_rewards - : {:?}", e))?;
}
