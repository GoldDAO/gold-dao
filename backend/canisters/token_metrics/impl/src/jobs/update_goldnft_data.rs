use canister_time::{ run_now_then_interval, MINUTE_IN_MS };
use std::time::Duration;
use types::Milliseconds;
use crate::state::{ mutate_state, read_state };

const REFRESH_GOLD_SUPPLY_INTERVAL: Milliseconds = 10 * MINUTE_IN_MS;

pub fn start_job() {
    run_now_then_interval(Duration::from_millis(REFRESH_GOLD_SUPPLY_INTERVAL), run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

async fn run_async() {
    let gold_nft_canisters = read_state(|s| s.data.gold_nft_canisters.clone());
    let mut total_grams: u64 = 0;

    for (gold_nft_canister_id, weight) in gold_nft_canisters {
        let total_supply: u64 = match
            canister_client::make_c2c_call(
                gold_nft_canister_id,
                "dip721_total_supply",
                {},
                ::candid::encode_one,
                |r| { ::candid::decode_one(r) }
            ).await
        {
            Ok(val) => { val }
            Err(err) => {
                let message: String = format!(
                    "The canister_client::make_c2c_call resulted into error : {err:?}"
                );
                ic_cdk::api::print(message);
                return ();
            }
        };

        total_grams += total_supply * weight;
    }

    mutate_state(|state| {
        state.data.total_gold_grams = total_grams;
    });
}
