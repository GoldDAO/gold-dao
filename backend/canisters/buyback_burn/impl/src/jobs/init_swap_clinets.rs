use crate::state::{ mutate_state, read_state };
use crate::utils::{ retry_with_attempts, RETRY_DELAY };
use candid::{ Nat, Principal };
use canister_tracing_macros::trace;
use tracing::{ error, info };
use crate::types::SwapClient;
use icpswap_factory_canister::ResultLowercase;
use futures::future::join_all;
use std::time::Duration;

const MAX_ATTEMPTS: u8 = 1;

pub fn start_job() {
    ic_cdk_timers::set_timer(Duration::ZERO, run);
}

pub fn run() {
    ic_cdk::spawn(run_async());
}

#[trace]
async fn run_async() {
    if
        let Err(err) = retry_with_attempts(MAX_ATTEMPTS, RETRY_DELAY, || async {
            init_all_swap_clients().await
        }).await
    {
        error!("Failed to init swap clients after {} attempts: {:?}", MAX_ATTEMPTS, err);
    }
}

pub async fn init_all_swap_clients() -> Result<(), String> {
    let icp_swap_canister_id = read_state(|state| state.data.icp_swap_canister_id);
    let mut swap_clients = read_state(|state| state.data.swap_clients.clone());

    let futures: Vec<_> = swap_clients
        .iter_mut()
        .map(|swap_client| { init_swap_client(swap_client.as_mut(), icp_swap_canister_id) })
        .collect();
    let results = join_all(futures).await;

    let mut error_messages = Vec::new();
    for result in results {
        if let Err(e) = result {
            error_messages.push(e);
        }
    }

    if error_messages.is_empty() {
        info!("Successfully inited all swap clients");
        let _ = mutate_state(|state| {
            state.data.swap_clients = swap_clients;
        });
        Ok(())
    } else {
        error!("Failed to init some swap clients:\n{}", error_messages.join("\n"));
        Err("Failed to init some swap clients".to_string())
    }
}

async fn init_swap_client(
    swap_client: &mut dyn SwapClient,
    icp_swap_canister_id: Principal
) -> Result<(), String> {
    let input_token = swap_client.get_config().input_token;
    let output_token = swap_client.get_config().output_token;

    // Fetch token standards asynchronously
    let input_token_standard = icrc_ledger_canister_c2c_client::icrc1_supported_standards(
        input_token.ledger_id
    ).await;
    let output_token_standard = icrc_ledger_canister_c2c_client::icrc1_supported_standards(
        output_token.ledger_id
    ).await;

    // Extract the first standard or handle errors if fetching fails
    let input_standard = match input_token_standard {
        Ok(standards) => standards.first().unwrap().name.clone(),
        Err(e) => {
            error!("Failed to fetch input token standard: {:?}", e);
            panic!("Failed to fetch input token standard: {:?}", e);
        }
    };

    let output_standard = match output_token_standard {
        Ok(standards) => standards.first().unwrap().name.clone(),
        Err(e) => {
            error!("Failed to fetch output token standard: {:?}", e);
            panic!("Failed to fetch input token standard: {:?}", e);
        }
    };

    let args = icpswap_factory_canister::get_pool::Args {
        token0: icpswap_factory_canister::get_pool::Token {
            address: input_token.ledger_id.to_string(),
            standard: input_standard,
        },
        token1: icpswap_factory_canister::get_pool::Token {
            address: output_token.ledger_id.to_string(),
            standard: output_standard,
        },
        // NOTE: it's always 3000: https://github.com/ICPSwap-Labs/docs/blob/main/01.SwapFactory/01.Searching_a_Pool.md
        fee: Nat::from(3000_u64),
    };

    let icpswap_swap_pool_id = icpswap_factory_canister_c2c_client
        ::get_pool(icp_swap_canister_id, &args).await
        .unwrap();

    match icpswap_swap_pool_id {
        ResultLowercase::Ok(pool_id) => {
            swap_client.set_swap_canister_id(pool_id.canister_id);
            Ok(())
        }
        ResultLowercase::Err(e) => {
            error!("Failed to fetch input token standard: {:?}", e);
            Err("Failed to fetch input token standard".to_string())
        }
    }
}
