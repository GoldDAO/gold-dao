use crate::logs::TRACE_XRC;
use crate::numeric::GoldPrice;
use crate::state::mutate_state;
use crate::SEC_NANOS;
use ic_canister_log::log;
use ic_xrc_types::GetExchangeRateResult;
use std::time::Duration;

pub const FETCHING_BTC_RATE_INTERVAL: Duration = Duration::from_secs(60);

pub async fn fetch_gold_price() -> bool {
    let mut has_fetched = false;
    match crate::management::fetch_gold_price().await {
        Ok(call_result) => match call_result {
            GetExchangeRateResult::Ok(exchange_rate_result) => {
                log!(
                    TRACE_XRC,
                    "[FetchPrice] fetched new gold rate: {} with timestamp: {}",
                    exchange_rate_result.rate,
                    exchange_rate_result.timestamp
                );

                let mut rate = exchange_rate_result.rate;
                let decimals = exchange_rate_result.metadata.decimals;
                if decimals > 8 {
                    for _ in 0..(decimals - 8) {
                        rate /= 10;
                    }
                } else {
                    for _ in 0..(8 - decimals) {
                        rate *= 10;
                    }
                }

                // 1 PAXG = 31.1034768g of gold
                // 1 GLDT = 0.01g of gold
                // 1 GLDT = 1 PAXG * 0.0003215074656862798
                let gldt_rate = (rate as f64 * 0.0003215074656862798) as u64;

                mutate_state(|s| {
                    s.one_centigram_of_gold_price = GoldPrice::from_e8s(gldt_rate);
                    s.last_rate_fetched_timestamp_nanos =
                        exchange_rate_result.timestamp * SEC_NANOS;
                });

                has_fetched = true;
            }
            GetExchangeRateResult::Err(error) => {
                ic_canister_log::log!(
                    TRACE_XRC,
                    "[FetchPrice] failed to call XRC canister with error: {error:?}"
                );
            }
        },
        Err(error) => ic_canister_log::log!(
            TRACE_XRC,
            "[FetchPrice] failed to call XRC canister with error: {error}"
        ),
    }

    has_fetched
}
