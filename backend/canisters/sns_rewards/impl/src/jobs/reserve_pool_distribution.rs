/*!
# Reserve pool distribution

- fn distribute_reserve_pool
transfers tokens from reserve pool to the reward pool on a daily basis.

- Sub accounts
reward pool - [0u8;32] -> holds ICP, OGY, GLDGov pre distribution
reserve pool - [1,0,0,0...etc]

- amount to transfer is given by a setting that can be updated via a proposal. for testing this is set to  40_000_000 / 365.25

*/

use crate::{
    model::payment_processor::{
        MaturityDelta,
        Payment,
        PaymentRound,
        PaymentRoundStatus,
        PaymentStatus,
    },
    state::{ mutate_state, read_state },
    utils::transfer_token,
};
use candid::{ Nat, Principal };
use canister_time::{ run_interval, DAY_IN_MS, WEEK_IN_MS };
use futures::{ future::{ err, join_all }, Future };
use ic_ledger_types::DEFAULT_SUBACCOUNT;
use icrc_ledger_types::icrc1::account::{ Account, Subaccount };
use sns_governance_canister::types::NeuronId;
use std::time::Duration;
use tracing::{ debug, error, info };
use types::{ Milliseconds, TokenSymbol };

const DISTRIBUTION_INTERVAL: Milliseconds = DAY_IN_MS;

pub fn start_job() {
    run_interval(Duration::from_millis(DISTRIBUTION_INTERVAL), run_distribution);
}

pub fn run_distribution() {
    ic_cdk::spawn(distribute_reserve_pool())
}

pub async fn distribute_reserve_pool() {
    info!("RESERVE DISTRIBUTION - START - retry attempt");

    let reserve_pool_sub_account: Subaccount = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let reward_pool_account = Account {
        owner: ic_cdk::api::id(),
        subaccount: Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ] as Subaccount),
    };
    let ammount = Nat::from(10_951_400_000_000u64); // TODO - we can get this from state which is changed via a proposal.
    let token = TokenSymbol::parse("GLDGov");
    let gld_gov_token_info = read_state(|s| s.data.tokens.get(token));
    transfer_token(reserve_pool_sub_account, reward_pool_account, ledger_id, amount);

    info!("RESERVE DISTRIBUTION - FINISH");
}

#[cfg(test)]
mod tests {
    #[test]
    fn mock_test() {}
}
