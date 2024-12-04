use crate::state::read_state;
use crate::vault::FeeBucket;
use ic_cdk::query;
use icrc_ledger_types::icrc1::account::Account;
use usdg_minter_api::{ApiFeeBucket, ApiVault};

#[query]
fn get_vaults_by_account(account: Option<Account>) -> Vec<ApiVault> {
    let account = account.unwrap_or(ic_cdk::caller().into());
    read_state(|s| {
        s.get_vaults_by_account(account)
            .into_iter()
            .map(|vault| {
                let fee_bucket = match vault.fee_bucket {
                    FeeBucket::Low => ApiFeeBucket::Low,
                    FeeBucket::Medium => ApiFeeBucket::Medium,
                    FeeBucket::High => ApiFeeBucket::High,
                };
                ApiVault {
                    vault_id: vault.vault_id,
                    owner: vault.owner,
                    borrowed_amount: vault.borrowed_amount.0,
                    margin_amount: vault.margin_amount.0,
                    fee_bucket,
                }
            })
            .collect()
    })
}
