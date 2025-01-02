use crate::state::read_state;
use crate::vault::FeeBucket;
use ic_canisters_http_types::{HttpRequest, HttpResponse, HttpResponseBuilder};
use ic_cdk::query;

#[query(hidden = true)]
fn http_request(req: HttpRequest) -> HttpResponse {
    use ic_metrics_encoder::MetricsEncoder;
    if ic_cdk::api::data_certificate().is_none() {
        ic_cdk::trap("update call rejected");
    }

    if req.path() == "/metrics" {
        let mut writer = MetricsEncoder::new(vec![], ic_cdk::api::time() as i64 / 1_000_000);

        fn encode_metrics(w: &mut MetricsEncoder<Vec<u8>>) -> std::io::Result<()> {
            const WASM_PAGE_SIZE_IN_BYTES: f64 = 65536.0;
            read_state(|s| {
                w.gauge_vec("cycle_balance", "Cycle balance of this canister.")?
                    .value(
                        &[("canister", "usdg-minter")],
                        ic_cdk::api::canister_balance128() as f64,
                    )?;
                w.encode_gauge(
                    "stable_memory_bytes",
                    ic_cdk::api::stable::stable_size() as f64 * WASM_PAGE_SIZE_IN_BYTES,
                    "Size of the stable memory allocated by this canister.",
                )?;
                w.encode_gauge(
                    "vault_count",
                    s.vault_id_to_vault.len() as f64,
                    "Count of open vaults.",
                )?;
                w.encode_gauge(
                    "pending_transfer_count",
                    s.pending_transfers.len() as f64,
                    "Count of pending transfers.",
                )?;
                w.encode_gauge(
                    "one_centigram_of_gold_price",
                    s.one_centigram_of_gold_price.to_f64(),
                    "Price of one centigram of gold.",
                )?;
                w.encode_gauge(
                    "total_gldt_margin",
                    s.total_gldt_margin().to_f64(),
                    "Total GLDT margin.",
                )?;
                w.encode_gauge(
                    "total_usdg_debt",
                    s.total_usdg_debt().to_f64(),
                    "Total USDG debt.",
                )?;
                w.encode_gauge(
                    "liquidation_pool_total_usdg_staked",
                    s.total_usdg_in_liquidation_pool().to_f64(),
                    "Total USDG in the liquidation pool.",
                )?;
                w.encode_gauge(
                    "liquidation_pool_stakers",
                    s.liquidation_pool.len() as f64,
                    "Count of users in staking USDG in the liquidation pool.",
                )?;
                w.encode_gauge(
                    "liquidation_pool_total_gldt_returns",
                    s.total_gldt_in_returns().to_f64(),
                    "Total GLDT in claimable returns the liquidation pool.",
                )?;
                w.encode_gauge(
                    "interest_rate_low",
                    *s.interest_rates.get(&FeeBucket::Low).unwrap(),
                    "Interest rate of the low bucket.",
                )?;
                w.encode_gauge(
                    "interest_rate_medium",
                    *s.interest_rates.get(&FeeBucket::Medium).unwrap(),
                    "Interest rate of the medium bucket.",
                )?;
                w.encode_gauge(
                    "interest_rate_high",
                    *s.interest_rates.get(&FeeBucket::High).unwrap(),
                    "Interest rate of the high bucket.",
                )?;
                w.encode_gauge(
                    "reserve_usdg",
                    s.reserve_usdg.to_f64(),
                    "USDG sitting in reserve.",
                )?;

                Ok(())
            })
        }

        match encode_metrics(&mut writer) {
            Ok(()) => HttpResponseBuilder::ok()
                .header("Content-Type", "text/plain; version=0.0.4")
                .with_body_and_content_length(writer.into_inner())
                .build(),
            Err(err) => {
                HttpResponseBuilder::server_error(format!("Failed to encode metrics: {}", err))
                    .build()
            }
        }
    } else if req.path() == "/logs" {
        use crate::logs::{Log, Priority};
        use serde_json;
        use std::str::FromStr;

        let max_skip_timestamp = match req.raw_query_param("time") {
            Some(arg) => match u64::from_str(arg) {
                Ok(value) => value,
                Err(_) => {
                    return HttpResponseBuilder::bad_request()
                        .with_body_and_content_length("failed to parse the 'time' parameter")
                        .build()
                }
            },
            None => 0,
        };

        let mut entries: Log = Default::default();

        match req.raw_query_param("priority") {
            Some(priority_str) => match Priority::from_str(priority_str) {
                Ok(priority) => match priority {
                    Priority::Info => entries.push_logs(Priority::Info),
                    Priority::TraceXrc => entries.push_logs(Priority::TraceXrc),
                    Priority::Debug => entries.push_logs(Priority::Debug),
                },
                Err(_) => entries.push_all(),
            },
            None => entries.push_all(),
        }

        entries
            .entries
            .retain(|entry| entry.timestamp >= max_skip_timestamp);
        let mut entries_bytes: Vec<u8> = serde_json::to_string(&entries)
            .unwrap_or_default()
            .into_bytes();

        // Truncate bytes to avoid having more than 3MB response.
        let max_size_bytes: usize = 3_000_000;
        entries_bytes.truncate(max_size_bytes);

        HttpResponseBuilder::ok()
            .header("Content-Type", "application/json; charset=utf-8")
            .with_body_and_content_length(entries_bytes)
            .build()
    } else {
        HttpResponseBuilder::not_found().build()
    }
}
