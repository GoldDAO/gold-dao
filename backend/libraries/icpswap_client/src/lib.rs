use candid::Nat;
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_ledger_types::Subaccount;
use icpswap_swap_pool_canister::{ICPSwapError, ICPSwapResult};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use types::{CanisterId, ICPSwapTokenInfo};

// NOTE: we use one ICPSwapClient to swap concrete token pair
#[derive(Serialize, Deserialize)]
pub struct ICPSwapClient {
    this_canister_id: CanisterId,
    swap_canister_id: CanisterId,
    token0: ICPSwapTokenInfo,
    token1: ICPSwapTokenInfo,
    // TODO: zero_for_one means which token is sold. There could be another neat solution
    zero_for_one: bool,
}

impl ICPSwapClient {
    pub fn new(
        this_canister_id: CanisterId,
        swap_canister_id: CanisterId,
        token0: ICPSwapTokenInfo,
        token1: ICPSwapTokenInfo,
        zero_for_one: bool,
    ) -> Self {
        ICPSwapClient {
            this_canister_id,
            swap_canister_id,
            token0,
            token1,
            zero_for_one,
        }
    }

    pub fn deposit_account(&self) -> Account {
        Account {
            owner: self.swap_canister_id,
            // NOTE: in open-chat we can see another convertation function,
            // but it seems like From trait could be used here
            subaccount: Some(Subaccount::from(self.this_canister_id).0),
        }
    }

    // NOTE: ICPSwap API - https://dashboard.internetcomputer.org/canister/7eikv-2iaaa-aaaag-qdgwa-cai
    pub async fn deposit(&self, amount: u128) -> CallResult<u128> {
        let token = self.input_token();
        let args = icpswap_swap_pool_canister::deposit::Args {
            token: token.ledger.to_string(),
            amount: amount.into(),
            fee: token.fee.into(),
        };
        match icpswap_swap_pool_canister_c2c_client::deposit(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_deposited) => Ok(nat_to_u128(amount_deposited)),
            ICPSwapResult::Err(error) => Err(convert_error(error)),
        }
    }

    pub async fn swap(
        &self,
        amount: u128,
        min_amount_out: u128,
    ) -> CallResult<Result<u128, String>> {
        let args = icpswap_swap_pool_canister::swap::Args {
            operator: self.this_canister_id,
            amount_in: amount.to_string(),
            zero_for_one: self.zero_for_one,
            amount_out_minimum: min_amount_out.to_string(),
        };
        match icpswap_swap_pool_canister_c2c_client::swap(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_out) => Ok(Ok(nat_to_u128(amount_out))),
            ICPSwapResult::Err(error) => Ok(Err(format!("{error:?}"))),
        }
    }

    // TODO: try here swap again (?)
    //
    pub async fn withdraw(&self, successful_swap: bool, amount: u128) -> CallResult<u128> {
        let token = if successful_swap {
            self.output_token()
        } else {
            self.input_token()
        };
        let args = icpswap_swap_pool_canister::withdraw::Args {
            token: token.ledger.to_string(),
            amount: amount.into(),
            fee: token.fee.into(),
        };
        match icpswap_swap_pool_canister_c2c_client::withdraw(self.swap_canister_id, &args).await? {
            ICPSwapResult::Ok(amount_out) => Ok(nat_to_u128(amount_out)),
            ICPSwapResult::Err(error) => Err(convert_error(error)),
        }
    }

    fn input_token(&self) -> &ICPSwapTokenInfo {
        if self.zero_for_one {
            &self.token0
        } else {
            &self.token1
        }
    }

    fn output_token(&self) -> &ICPSwapTokenInfo {
        if self.zero_for_one {
            &self.token1
        } else {
            &self.token0
        }
    }
}

fn nat_to_u128(value: Nat) -> u128 {
    value.0.try_into().unwrap()
}

fn convert_error(error: ICPSwapError) -> (RejectionCode, String) {
    (RejectionCode::Unknown, format!("{error:?}"))
}
