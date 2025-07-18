use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::processing_rewards_pool::PROCESS_REWARDS_THRESHOLD;
use candid::CandidType;
use candid::Nat;
use candid::Principal;
use gldt_stake_common::accounts::UNALLOCATED_REWARDS_POOL;
use icrc_ledger_canister_c2c_client::{icrc1_balance_of, icrc1_transfer};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::{Deserialize, Serialize};
use utils::numeric::Percentage;

const REWARDS_PERCENTAGE: u8 = 33;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UnallocatedRewardsPool {
    pub state: UnallocatedRewardsState,
}

impl UnallocatedRewardsPool {
    pub fn new_unallocated() -> Self {
        Self {
            state: UnallocatedRewardsState::default(),
        }
    }

    pub fn current_state(&self) -> &UnallocatedRewardsState {
        &self.state
    }
}

impl UnallocatedRewards for UnallocatedRewardsPool {}

#[allow(async_fn_in_trait)]
pub trait UnallocatedRewards {
    const SUBACCOUNT: Subaccount = UNALLOCATED_REWARDS_POOL;

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

    async fn transfer_part_of_rewards(
        &self,
        token_ledger: Principal,
        fee: Nat,
    ) -> Result<Nat, String> {
        let pool_balance = self.balance(token_ledger).await?;

        let percentage =
            Percentage::new(REWARDS_PERCENTAGE).map_err(|e| format!("Invalid percentage: {e}"))?;

        let amount_to_transfer = percentage.apply_to(&pool_balance) / 7_u64;

        if amount_to_transfer == 0_u64 {
            let msg = format!(
            "Calculated transfer amount is zero (pool_balance: {}, REWARDS_PERCENTAGE: {}). Skipping transfer.",
            pool_balance, REWARDS_PERCENTAGE
        );
            return Err(msg);
        }

        if fee > pool_balance {
            let msg = format!(
            "Calculated transfer amount is less than fee (pool_balance: {}, fee: {}). Skipping transfer.",
            pool_balance, fee
        );
            return Err(msg);
        }

        if PROCESS_REWARDS_THRESHOLD > pool_balance {
            let msg = format!(
            "Calculated transfer amount is less than the threshold (pool_balance: {}, threshold: {}). Skipping transfer.",
            pool_balance, PROCESS_REWARDS_THRESHOLD
        );
            return Err(msg);
        }

        let result = match icrc1_transfer(
            token_ledger,
            &TransferArg {
                from_subaccount: None,
                to: ProcessingRewardsPool::account(),
                fee: None,
                created_at_time: None,
                memo: None,
                amount: amount_to_transfer.clone(),
            },
        )
        .await
        {
            Ok(Ok(_)) => Ok(amount_to_transfer),
            Ok(Err(e)) => {
                let err_msg = format!("Transfer error: {:?}", e);
                Err(err_msg)
            }
            Err(e) => {
                let err_msg = format!("Transfer call failed: {:?}", e);
                Err(err_msg)
            }
        };

        result
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, CandidType)]
pub enum UnallocatedRewardsState {
    #[default]
    Awaiting,
    // Claiming rewards from the sns_rewards canister.
    Claiming,
    TransferringToProcessingPool,
    Error(String),
}

impl UnallocatedRewardsPool {
    pub fn is_awaiting(&self) -> bool {
        matches!(self.state, UnallocatedRewardsState::Awaiting)
    }

    pub fn transition_to_claiming(&mut self) {
        self.state = match self.state {
            UnallocatedRewardsState::Awaiting => UnallocatedRewardsState::Claiming,
            _ => UnallocatedRewardsState::Error(format!(
                "Invalid transition from {:?} to Claiming",
                self.state
            )),
        }
    }

    pub fn transition_to_transferring(&mut self) {
        self.state = match self.state {
            UnallocatedRewardsState::Claiming => {
                UnallocatedRewardsState::TransferringToProcessingPool
            }
            _ => UnallocatedRewardsState::Error(format!(
                "Invalid transition from {:?} to TransferringToProcessingPool",
                self.state
            )),
        }
    }

    pub fn transition_to_error(&mut self, msg: String) {
        self.state = UnallocatedRewardsState::Error(msg);
    }

    pub fn transition_to_awaiting(&mut self) {
        self.state = UnallocatedRewardsState::Awaiting;
    }
}
