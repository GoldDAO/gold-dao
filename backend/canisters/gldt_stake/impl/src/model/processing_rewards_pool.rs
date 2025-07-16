use crate::model::allocated_rewards_pool::AllocatedRewards;
use crate::model::allocated_rewards_pool::AllocatedRewardsPool;
use crate::model::allocated_rewards_pool::ALLOCATE_REWARDS_THRESHOLD;
use candid::Nat;
use candid::Principal;
use gldt_stake_common::accounts::PROCESSING_REWARDS_POOL;
use icrc_ledger_canister_c2c_client::icrc1_balance_of;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::{Deserialize, Serialize};

pub const PROCESS_REWARDS_THRESHOLD: u64 = 100_000_000_u64;

#[allow(async_fn_in_trait)]
pub trait ProcessingRewards {
    const SUBACCOUNT: Subaccount = PROCESSING_REWARDS_POOL;

    fn account() -> Account {
        Account {
            owner: ic_cdk::api::canister_self(),
            subaccount: Some(Self::SUBACCOUNT),
        }
    }

    async fn balance(&self, token_ledger: Principal) -> Result<Nat, String> {
        match icrc1_balance_of(token_ledger, Self::account()).await {
            Ok(balance) => Ok(balance),
            Err(e) => Err(format!("RewardPool : fetch_pool_balance error: {e:?}")),
        }
    }

    async fn transfer_rewards(&self, token_ledger: Principal, amount: Nat) -> Result<Nat, String> {
        let pool_balance = self.balance(token_ledger).await?;

        if amount == 0_u64 {
            return Err("Transfer amount is zero. Skipping transfer.".to_string());
        }

        if amount > pool_balance {
            return Err(format!(
                "Transfer amount ({}) exceeds pool balance ({}). Skipping transfer.",
                amount, pool_balance
            ));
        }

        if ALLOCATE_REWARDS_THRESHOLD > pool_balance {
            let msg = format!(
            "Calculated transfer amount is less than the threshold (pool_balance: {}, threshold: {}). Skipping transfer.",
            pool_balance, ALLOCATE_REWARDS_THRESHOLD
        );
            return Err(msg);
        }

        // Perform transfer
        match icrc1_transfer(
            token_ledger,
            &TransferArg {
                from_subaccount: None,
                to: AllocatedRewardsPool::account(),
                fee: None,
                created_at_time: None,
                memo: None,
                amount: amount.clone(),
            },
        )
        .await
        {
            Ok(Ok(_)) => Ok(amount),
            Ok(Err(e)) => Err(format!("Transfer error: {e:?}")),
            Err(e) => Err(format!("Transfer call failed: {e:?}")),
        }
    }
}

impl ProcessingRewards for ProcessingRewardsPool {}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessingRewardsPool {
    pub state: ProcessingRewardsState,
}

impl ProcessingRewardsPool {
    pub fn new_processing() -> Self {
        Self {
            state: ProcessingRewardsState::default(),
        }
    }

    pub fn is_awaiting(&self) -> bool {
        matches!(self.state, ProcessingRewardsState::Awaiting)
    }

    pub fn current_state(&self) -> &ProcessingRewardsState {
        &self.state
    }

    pub fn transition_to_transferring(&mut self) {
        self.state = match self.state {
            ProcessingRewardsState::Awaiting => {
                ProcessingRewardsState::TransferringToAllocatingPool
            }
            _ => ProcessingRewardsState::Error(format!(
                "Invalid transition from {:?} to TransferringToAllocatingPool",
                self.state
            )),
        }
    }

    pub fn transition_to_error(&mut self, msg: String) {
        self.state = ProcessingRewardsState::Error(msg);
    }

    pub fn transition_to_awaiting(&mut self) {
        self.state = ProcessingRewardsState::Awaiting;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum ProcessingRewardsState {
    #[default]
    Awaiting,
    TransferringToAllocatingPool,
    Error(String),
}
