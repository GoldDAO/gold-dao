use crate::guards::caller_is_governance_principal;
use crate::utils::transfer_token;
use bity_ic_canister_tracing_macros::trace;
use ic_cdk::update;
use icrc_ledger_types::icrc1::account::Account;
pub use sns_rewards_api_canister::burn_goldao_reserve_pool::{
    Args as BurnGoldaoReservePoolArgs, Response as BurnGoldaoReservePoolResponse,
};
use sns_rewards_api_canister::subaccounts::RESERVE_POOL_SUB_ACCOUNT;
use tracing::error;
use tracing::info;
use types::ledger_id;

#[update(guard = "caller_is_governance_principal")]
#[trace]
pub async fn burn_goldao_reserve_pool(
    _args: BurnGoldaoReservePoolArgs,
) -> BurnGoldaoReservePoolResponse {
    match burn_goldao_reserve_pool_impl().await {
        Ok(_) => BurnGoldaoReservePoolResponse::Success,
        Err(err) => BurnGoldaoReservePoolResponse::InternalError(err),
    }
}

pub(crate) async fn burn_goldao_reserve_pool_impl() -> Result<(), String> {
    let goldao_ledger_id = ledger_id!(GOLDAO);

    let amount_to_burn = icrc_ledger_canister_c2c_client::icrc1_balance_of(
        goldao_ledger_id,
        &(Account {
            owner: ic_cdk::api::canister_self(),
            subaccount: Some(RESERVE_POOL_SUB_ACCOUNT),
        }),
    )
    .await
    .map_err(|e| format!("Failed to fetch token balance: {:?}", e))?;

    let minting_account =
        match icrc_ledger_canister_c2c_client::icrc1_minting_account(goldao_ledger_id).await {
            Ok(Some(account)) => Ok(account),
            Ok(None) => Err("Minting account is None".to_string()),
            Err(e) => Err(format!(
                "Failed to get minting account (in order to burn tokens): {:?}",
                e
            )),
        }?;

    match transfer_token(
        RESERVE_POOL_SUB_ACCOUNT,
        minting_account,
        goldao_ledger_id,
        amount_to_burn.clone(),
    )
    .await
    {
        Ok(_) => {
            info!(
                "SUCCESS : {:?} GOLDAO tokens burned successfully",
                amount_to_burn
            );
        }
        Err(e) => {
            error!(
                "ERROR : GOLDAO failed to burn from reserve pool with error : {:?}",
                e
            );
        }
    }

    Ok(())
}
