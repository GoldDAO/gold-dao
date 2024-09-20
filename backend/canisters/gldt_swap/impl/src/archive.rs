use candid::{ Encode, Nat, Principal };
use gldt_swap_common::archive::ArchiveCanister;
use gldt_swap_api_archive::init_archive::InitArgArchive;
use gldt_swap_archive_c2c_client::get_archive_size;
use ic_cdk::api::management_canister::main::{
    canister_status,
    create_canister,
    install_code,
    start_canister,
    stop_canister,
    CanisterId,
    CanisterIdRecord,
    CanisterInstallMode,
    CanisterSettings,
    CreateCanisterArgument,
    InstallCodeArgument,
    LogVisibility,
};
use tracing::debug;
use utils::{ env::Environment, retry_async::retry_async };

use crate::state::{ mutate_state, read_state };

const ARCHIVE_WASM: &[u8] = include_bytes!("../../archive/wasm/gldt_swap_archive_canister.wasm.gz");

pub async fn check_storage_and_create_archive() -> Result<(), ()> {
    // check if the capacity is
    let current_swap_index = read_state(|s| s.data.swaps.get_current_swap_index());
    if
        let Some(current_archive) = read_state(|s|
            s.data.swaps.get_archive_canisters().last().cloned()
        )
    {
        if is_archive_canister_at_threshold(&current_archive).await {
            let archive_principal = match create_archive_canister().await {
                Ok(principal) => { principal }
                Err(e) => {
                    debug!(e);
                    return Err(());
                }
            };
            mutate_state(|s|
                s.data.swaps.set_new_archive_canister(ArchiveCanister {
                    canister_id: archive_principal,
                    start_index: current_swap_index,
                    end_index: None,
                })
            );
            // new archive created
            return Ok(());
        } else {
            // we still have room
            return Ok(());
        }
    }
    // no archive canisters
    Err(())
}

pub async fn create_archive_canister() -> Result<Principal, String> {
    let this_canister_id = read_state(|s| s.env.canister_id());
    let mut controllers = get_canister_controllers(this_canister_id).await?;
    controllers.push(ic_cdk::api::id());

    let initial_cycles = 2_000_000_000_000u64; // 2 Trillion cycles
    // Define the initial settings for the new canister
    let settings = CanisterSettings {
        controllers: Some(controllers), // Ensure the current canister is a controller
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: Some(Nat::from(initial_cycles)),
        log_visibility: Some(LogVisibility::Public),
        wasm_memory_limit: None, // use default of 3GB
    };
    // Step 1: Create the canister
    let canister_id = match
        retry_async(
            ||
                create_canister(
                    CreateCanisterArgument {
                        settings: Some(settings.clone()),
                    },
                    initial_cycles as u128
                ),
            3
        ).await
    {
        Ok(canister) => { canister.0.canister_id }
        Err(e) => {
            return Err(format!("ERROR : failed to create a canister id with error - {e:?}"));
        }
    };
    let mut current_auth_prins = read_state(|s| s.data.authorized_principals.clone());
    let test_mode = read_state(|s| s.env.is_test_mode());
    let version = read_state(|s| s.data.version.clone());
    current_auth_prins.push(this_canister_id);

    let init_args = match
        Encode!(
            &(InitArgArchive {
                version: version,
                authorized_principals: current_auth_prins,
                test_mode: test_mode,
            })
        )
    {
        Ok(encoded_init_args) => encoded_init_args,
        Err(e) => {
            return Err(format!("ERROR : failed to create init args with error - {e}"));
        }
    };

    // Step 2: Install the Wasm module to the newly created canister
    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id,
        wasm_module: ARCHIVE_WASM.to_vec(),
        arg: init_args,
    };

    match retry_async(|| install_code(install_args.clone()), 3).await {
        Ok(_) => Ok(canister_id),
        Err((code, msg)) => {
            return Err(format!("ERROR : {code:?} - {msg}"));
        }
    }
}

pub async fn is_archive_canister_at_threshold(archive: &ArchiveCanister) -> bool {
    let res = retry_async(|| get_archive_size(archive.canister_id, &()), 3).await;
    let max_canister_archive_threshold = read_state(|s|
        s.data.max_canister_archive_threshold.clone()
    );
    match res {
        Ok(size) => { size >= max_canister_archive_threshold }
        Err(_) => { false }
    }
}

pub async fn update_archive_canisters() -> Result<(), Vec<String>> {
    let archive_canisters = read_state(|s| s.data.swaps.get_archive_canisters());
    let version = read_state(|s| s.data.version.clone());
    let test_mode = read_state(|s| s.env.is_test_mode());
    let mut current_auth_prins = read_state(|s| s.data.authorized_principals.clone());
    let this_canister_id = read_state(|s| s.env.canister_id());
    current_auth_prins.push(this_canister_id);

    let init_args = match
        Encode!(
            &(InitArgArchive {
                authorized_principals: current_auth_prins,
                test_mode: test_mode,
                version: version,
            })
        )
    {
        Ok(encoded_init_args) => encoded_init_args,
        Err(e) => {
            return Err(vec![format!("ERROR : failed to create init args with error - {e}")]);
        }
    };

    let mut canister_upgrade_errors = vec![];

    for archive in archive_canisters {
        match
            retry_async(
                ||
                    stop_canister(CanisterIdRecord {
                        canister_id: archive.canister_id,
                    }),
                3
            ).await
        {
            Ok(_) => {}
            Err(e) => {
                canister_upgrade_errors.push(
                    format!(
                        "ERROR: archive upgrade :: archive with principal : {} failed to stop with error {:?}",
                        archive.canister_id,
                        e
                    )
                );
                continue;
            }
        }

        let result = {
            let init_args = init_args.clone();
            let wasm_module = ARCHIVE_WASM.to_vec();

            let install_args = InstallCodeArgument {
                mode: CanisterInstallMode::Upgrade(None),
                canister_id: archive.canister_id,
                wasm_module,
                arg: init_args,
            };
            retry_async(|| install_code(install_args.clone()), 3).await
        };

        match result {
            Ok(_) => {
                match
                    retry_async(
                        ||
                            start_canister(CanisterIdRecord {
                                canister_id: archive.canister_id,
                            }),
                        3
                    ).await
                {
                    Ok(_) => {}
                    Err(e) => {
                        canister_upgrade_errors.push(
                            format!(
                                "ERROR: archive upgrade :: archive with principal : {} failed to start with error {:?}",
                                archive.canister_id,
                                e
                            )
                        );
                    }
                }
            }
            Err(e) => {
                canister_upgrade_errors.push(
                    format!(
                        "ERROR: archive upgrade :: archive with principal : {} failed to install upgrade {:?}",
                        archive.canister_id,
                        e
                    )
                );
            }
        }
    }

    if canister_upgrade_errors.len() > 0 {
        return Err(canister_upgrade_errors);
    } else {
        Ok(())
    }
}

async fn get_canister_controllers(canister_id: CanisterId) -> Result<Vec<Principal>, String> {
    match retry_async(|| canister_status(CanisterIdRecord { canister_id }), 3).await {
        Ok(res) => Ok(res.0.settings.controllers),
        Err(e) => { Err(format!("Failed to get canister status: {:?}", e)) }
    }
}
