use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::{Account, Subaccount},
    transfer::TransferArg,
};
use sns_governance_canister::types::NeuronId;
use tracing::error;

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

pub async fn fetch_neuron_reward_balance(
    ledger_canister_id: Principal,
    ogy_sns_rewards_canister_id: Principal,
    neuron_id: &NeuronId,
) -> Nat {
    match icrc_ledger_canister_c2c_client::icrc1_balance_of(
        ledger_canister_id,
        &(Account {
            owner: ogy_sns_rewards_canister_id,
            subaccount: Some(neuron_id.into()),
        }),
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            error!(
                "Failed to fetch token balance of ledger canister id {} with ERROR : {:?}",
                ledger_canister_id, e
            );
            Nat::from(0u64)
        }
    }
}
