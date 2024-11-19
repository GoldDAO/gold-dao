use crate::state::{mutate_state, read_state};
use candid::{Nat, Principal};
use canister_time::{run_interval, timestamp_nanos};
use futures::future::join_all;
use gldt_swap_common::{
    gldt::{GLDT_LEDGER_FEE_ACCOUNT, GLDT_PRICE_RATIO, GLDT_SUBDIVIDABLE_BY},
    nft::NftCanisterConf,
    swap::{trace, MANAGE_GLDT_SUPPLY_INTERVAL, MANAGE_GLDT_SUPPLY_RETRY_DELAY},
};
use ic_cdk::spawn;
use ic_cdk_timers::set_timer;
use icrc_ledger_canister_c2c_client::{icrc1_total_supply, icrc1_transfer};
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{Memo, TransferArg},
};
use origyn_nft_reference_c2c_client::count_unlisted_tokens_of;
use std::time::Duration;
use tracing::info;
use utils::env::Environment;

pub fn start_job() {
    run_interval(
        Duration::from_millis(MANAGE_GLDT_SUPPLY_INTERVAL),
        spawn_gldt_supply_balancer,
    );
}

pub fn spawn_gldt_supply_balancer() {
    trace("MANAGE GLDT SUPPLY :: 0 INITIAL");
    if read_state(|s| s.data.is_gldt_supply_balancer_running) {
        trace("MANAGE GLDT SUPPLY :: 0 already running");
        return;
    } else {
        ic_cdk::spawn(gldt_supply_balancer(1))
    }
}

fn schedule_retry(attempt: u32) {
    if attempt < 4 {
        trace(&format!(
            "MANAGE GLDT SUPPLY :: retry with attempt {attempt}"
        ));
        set_timer(
            Duration::from_millis(MANAGE_GLDT_SUPPLY_RETRY_DELAY),
            move || {
                if read_state(|s| s.data.is_gldt_supply_balancer_running) {
                    trace(
                    &format!(
                        "MANAGE GLDT SUPPLY :: retry with attempt {attempt} is already running, exiting early"
                    )
                );
                    return ();
                }
                trace(&format!(
                    "MANAGE GLDT SUPPLY :: retry with attempt {attempt} will do attempt"
                ));
                spawn(gldt_supply_balancer(attempt));
            },
        );
    } else {
        trace("MANAGE GLDT SUPPLY :: 3 retries attempted. will not retry any more");
        info!("MANAGE GLDT SUPPLY :: 3 retries attempted. will not retry any more")
    }
}

async fn gldt_supply_balancer(attempt: u32) {
    trace("MANAGE GLDT SUPPLY :: gldt_supply_balancer called");
    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = true;
    });
    // if active swaps present we must NEVER proceed to balance gldt supply
    let active_swaps = read_state(|s| s.data.swaps.get_active_swaps());
    if active_swaps.len() > 0 {
        mutate_state(|s| {
            s.data.is_gldt_supply_balancer_running = false;
        });
        schedule_retry(attempt + 1);
        return ();
    }

    trace("MANAGE GLDT SUPPLY 1 :: will run");
    info!("MANAGE GLDT SUPPLY 1 :: will run");

    let gldt_ledger_id = read_state(|s| s.data.gldt_ledger_id);
    let owned_nfts = read_state(|s| s.data.canister_owned_nfts.clone());
    let active_nft_canisters = read_state(|s| s.data.gldnft_canisters.clone());

    info!("MANAGE GLDT SUPPLY 2 :: {owned_nfts:?}");
    // calculate expected vs real total supply
    let expected_gldt_supply_res = calculate_expected_gldt_supply(active_nft_canisters).await;
    let real_supply_res = icrc1_total_supply(gldt_ledger_id).await;

    trace(&format!(
        "MANAGE GLDT SUPPLY 3 :: expected {expected_gldt_supply_res:?}"
    ));
    info!("MANAGE GLDT SUPPLY 3 :: expected {expected_gldt_supply_res:?}");
    trace(&format!(
        "MANAGE GLDT SUPPLY 4 :: real supply {real_supply_res:?}"
    ));
    info!("MANAGE GLDT SUPPLY 4 :: real supply {real_supply_res:?}");

    match real_supply_res {
        Ok(real_supply) => {
            match expected_gldt_supply_res {
                Ok(expected_supply) => {
                    let diff = if expected_supply > real_supply {
                        expected_supply - real_supply
                    } else {
                        Nat::from(0u64)
                    };

                    if diff > Nat::from(0u64) {
                        mint_gldt_to_fee_account(diff).await;
                    }
                }
                Err(e) => {
                    // bad calculation
                    trace(&format!("MANAGE GLDT SUPPLY 6 :: {e:?}"));
                    info!("MANAGE GLDT SUPPLY 6 :: {e:?}");
                }
            }
        }
        Err(e) => {
            // bad call
            trace(&format!("MANAGE GLDT SUPPLY 7 :: {e:?}"));
            info!("MANAGE GLDT SUPPLY 7 :: {e:?}");
        }
    }

    mutate_state(|s| {
        s.data.is_gldt_supply_balancer_running = false;
    });
}

async fn calculate_expected_gldt_supply(
    nft_canisters: Vec<(Principal, NftCanisterConf, Option<Account>)>,
) -> Result<Nat, String> {
    let this_canister_id = read_state(|s| s.env.canister_id());

    let (futures, collections): (Vec<_>, Vec<_>) = nft_canisters
        .into_iter()
        .map(|(collection_id, conf, _)| {
            (
                async move {
                    count_unlisted_tokens_of(
                        collection_id.clone(),
                        &(Account {
                            owner: this_canister_id.clone(),
                            subaccount: None,
                        }),
                    )
                    .await
                },
                (conf.grams, collection_id),
            )
        })
        .unzip();

    let results = join_all(futures).await;

    let mut total_gldt_expected = Nat::from(0u64);
    for (result, collection) in results.into_iter().zip(collections.into_iter()) {
        match result {
            Ok(amount) => {
                mutate_state(|s| s.set_owned_nft(collection.1, collection.0, amount.clone()));
                total_gldt_expected += gldt_from_quantity_of_nfts(&amount, &collection.0);
            }
            Err(e) => {
                return Err(format!("{e:?}"));
            }
        }
    }

    Ok(total_gldt_expected)
}

fn gldt_from_quantity_of_nfts(amount: &Nat, weight: &u16) -> Nat {
    let gldt_per_nft =
        Nat::from(weight.clone()) * Nat::from(GLDT_PRICE_RATIO) * GLDT_SUBDIVIDABLE_BY;
    amount.clone() * gldt_per_nft
}

async fn mint_gldt_to_fee_account(amount: Nat) {
    let gldt_canister_id = read_state(|s| s.data.gldt_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());

    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: this_canister_id,
            subaccount: Some(GLDT_LEDGER_FEE_ACCOUNT),
        },
        amount: Nat::from(amount.clone()),
        fee: None,
        created_at_time: Some(timestamp_nanos()),
        memo: Some(Memo::from("AUTO_BALANCE_BURNT_GLDT".as_bytes().to_vec())),
    };

    match icrc1_transfer(gldt_canister_id, &args).await {
        Ok(res) => {
            match res {
                Ok(_) => {
                    // went okay
                    trace(&format!(
                        "MANAGE GLDT SUPPLY 9 :: successfully minted {amount}"
                    ));
                    info!("MANAGE GLDT SUPPLY 9 :: successfully minted {amount}");
                }
                Err(e) => {
                    // mitn failed
                    trace(&format!("MANAGE GLDT SUPPLY 10 :: failed {e:?}"));
                    info!("MANAGE GLDT SUPPLY 10 :: failed {e:?}");
                }
            }
        }
        Err(e) => {
            // call failed // network error
            trace(&format!("MANAGE GLDT SUPPLY 10 :: failed {e:?}"));
            info!("MANAGE GLDT SUPPLY 10 :: failed {e:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gldt_from_quantity_of_nfts_works_correctly() {
        let res = gldt_from_quantity_of_nfts(&Nat::from(100u64), &1);
        assert_eq!(res, Nat::from(1_000_000_000_000u64));

        let res = gldt_from_quantity_of_nfts(&Nat::from(55u64), &100);
        assert_eq!(res, Nat::from(55_000_000_000_000u64));

        let res = gldt_from_quantity_of_nfts(&Nat::from(0u64), &1000);
        assert_eq!(res, Nat::from(0u64))
    }
}
