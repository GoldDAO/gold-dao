use crate::model::processing_rewards_pool::ProcessingRewards;
use crate::model::processing_rewards_pool::ProcessingRewardsPool;
use crate::model::processing_rewards_pool::PROCESS_REWARDS_THRESHOLD;
use candid::{CandidType, Nat, Principal};
use gldt_stake_common::accounts::UNALLOCATED_REWARDS_POOL;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use icrc_ledger_canister_c2c_client::{icrc1_balance_of, icrc1_transfer};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::{Deserialize, Serialize};
use utils::numeric::Percentage;

pub const REWARDS_PERCENTAGE: u8 = 33;

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

    async fn balance(&self, token_ledger: Principal) -> Result<Nat, GeneralError> {
        match icrc1_balance_of(token_ledger, Self::account()).await {
            Ok(balance) => Ok(balance),
            Err(e) => Err(GeneralError::CallError(format!(
                "RewardPool : fetch_pool_balance error: {e:?}"
            ))),
        }
    }

    async fn transfer_part_of_rewards(
        &self,
        token_ledger: Principal,
        fee: Nat,
        apy_limited_amount: Option<Nat>,
    ) -> Result<Nat, GeneralError> {
        ic_cdk::println!(
        "[transfer_part_of_rewards] Starting transfer for ledger: {}, fee: {}, apy_limited_amount: {:?}",
        token_ledger,
        fee,
        apy_limited_amount
    );

        let pool_balance = self.balance(token_ledger).await?;

        let percentage = Percentage::new(REWARDS_PERCENTAGE).map_err(|e| {
            let msg = format!("Invalid percentage: {e}");
            GeneralError::InvalidPercentage(msg)
        })?;

        let percentage_based_amount = percentage.apply_to(&pool_balance) / 7_u64;

        let amount_to_transfer = if let Some(apy_limited_amount) = apy_limited_amount {
            std::cmp::min(percentage_based_amount.clone(), apy_limited_amount.clone())
        } else {
            percentage_based_amount
        };

        if amount_to_transfer == 0_u64 {
            let msg = format!(
            "Calculated transfer amount is zero (amount_to_transfer: {}, REWARDS_PERCENTAGE: {}). Skipping transfer.",
            pool_balance, REWARDS_PERCENTAGE
        );
            return Err(GeneralError::BalanceIsZero(msg));
        }

        if fee > amount_to_transfer {
            let msg = format!(
            "Calculated transfer amount is less than fee (amount_to_transfer: {}, fee: {}). Skipping transfer.",
            amount_to_transfer, fee
        );
            return Err(GeneralError::BalanceIsLowerThanFee(msg));
        }

        if PROCESS_REWARDS_THRESHOLD > amount_to_transfer {
            let msg = format!(
            "Calculated transfer amount is less than the threshold (amount_to_transfer: {}, threshold: {}). Skipping transfer.",
            amount_to_transfer, PROCESS_REWARDS_THRESHOLD
        );
            return Err(GeneralError::BalanceIsLowerThanFee(msg));
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
                Err(GeneralError::TransferError(err_msg))
            }
            Err(e) => {
                let err_msg = format!("Transfer call failed: {:?}", e);
                Err(GeneralError::TransferError(err_msg))
            }
        };

        result
    }
}

pub enum ProcessRewardsTransferError {
    InvalidPercentage,
    BalanceIsZero,
    BalanceIsLowerThanFee,
    UnexpectedError(String),
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
