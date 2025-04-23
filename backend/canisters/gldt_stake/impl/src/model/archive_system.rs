use candid::Nat;
use candid::{Encode, Principal};
use gldt_stake_api_archive::lifecycle::Args as ArgsArchive;
use gldt_stake_api_archive::{init::InitArgs, post_upgrade::UpgradeArgs};
use gldt_stake_archive_c2c_client::{archive_item, get_archive_size, ArchiveItemArgs};
use gldt_stake_common::archive::ArchiveCanister;
use gldt_stake_common::archive::ArchiveStatus;
use gldt_stake_common::archive::NewArchiveError;
use gldt_stake_common::trace;
use serde::{Deserialize, Serialize};
use tracing::debug;

use gldt_stake_common::stake_position::{StakePosition, StakePositionId};
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, install_code, start_canister, stop_canister, CanisterId,
    CanisterIdRecord, CanisterInstallMode, CanisterSettings, CreateCanisterArgument,
    InstallCodeArgument, LogVisibility,
};
use tracing::info;
use utils::{env::Environment, retry_async::retry_async};

use crate::state::{mutate_state, read_state};

const ARCHIVE_WASM: &[u8] =
    include_bytes!("../../../archive/wasm/gldt_stake_archive_canister.wasm.gz");

#[derive(Serialize, Deserialize, Clone)]
pub struct ArchiveSystem {
    pub archive_canisters: Vec<ArchiveCanister>,
    pub max_canister_archive_threshold: u128,
    pub archive_status: ArchiveStatus,
    pub new_archive_error: Option<NewArchiveError>,
    pub required_cycle_balance: Nat,
}

impl Default for ArchiveSystem {
    fn default() -> Self {
        Self {
            archive_canisters: Default::default(),
            archive_status: ArchiveStatus::Initializing,
            required_cycle_balance: Nat::default(),
            max_canister_archive_threshold: 300 * 1024 * 1024 * 1024_u128, // 300GB
            new_archive_error: None,
        }
    }
}

impl ArchiveSystem {
    pub fn set_archive_canisters(&mut self, archive_canisters: Vec<ArchiveCanister>) {
        self.archive_canisters = archive_canisters;
    }

    pub fn get_archive_canisters(&self) -> Vec<ArchiveCanister> {
        self.archive_canisters.clone()
    }

    pub fn get_active_archive_canister(&self) -> Option<ArchiveCanister> {
        self.archive_canisters
            .iter()
            .find(|arch| arch.active)
            .cloned()
    }

    pub fn set_new_archive_canister(&mut self, new_active_archive_canister: ArchiveCanister) {
        match self.archive_canisters.iter_mut().find(|arch| arch.active) {
            Some(active_archive) => {
                active_archive.active = false;
                trace(&format!(
                    "SET_NEW_ARCHIVE_CANISTER :: old archive has been set to inactive {:?}",
                    new_active_archive_canister
                ));
            }
            None => {
                info!("SET_NEW_ARCHIVE_CANISTER :: no active to make inactive present")
            }
        }
        info!(
            "SET_NEW_ARCHIVE_CANISTER :: new achive added {:?}",
            new_active_archive_canister,
        );
        self.archive_canisters.push(new_active_archive_canister);
    }

    pub fn set_archive_status(&mut self, status: ArchiveStatus) {
        self.archive_status = status;
    }

    pub fn get_archive_status(&self) -> ArchiveStatus {
        self.archive_status.clone()
    }

    pub fn get_total_archive_canisters(&self) -> usize {
        self.archive_canisters.len()
    }
}

pub async fn check_storage_and_create_archive() -> Result<(), ()> {
    // check if the capacity is
    if let Some(current_archive) =
        read_state(|s| s.data.archive_system.get_active_archive_canister())
    {
        if is_archive_canister_at_threshold(&current_archive).await {
            info!("ARCHIVE :: at capacity :: creating new archive canister");
            let archive_principal = match create_archive_canister().await {
                Ok(principal) => principal,
                Err(e) => {
                    debug!("{e:?}");
                    mutate_state(|s| {
                        s.data.archive_system.new_archive_error = Some(e);
                    });
                    return Err(());
                }
            };
            mutate_state(|s| {
                s.data
                    .archive_system
                    .set_new_archive_canister(ArchiveCanister {
                        canister_id: archive_principal,
                        active: true,
                    })
            });
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

pub async fn create_archive_canister() -> Result<Principal, NewArchiveError> {
    let this_canister_id = read_state(|s| s.env.canister_id());
    let test_mode = read_state(|s| s.env.is_test_mode());
    let mut controllers = get_canister_controllers(this_canister_id).await?;
    controllers.push(ic_cdk::api::id());

    let initial_cycles = if test_mode {
        5_000_000_000_000u64 // 2 Trillion cycles
    } else {
        10_000_000_000_000u64 // 2 Trillion cycles
    };

    let reserved_cycles = if test_mode {
        2_000_000_000_000u64 // 2 Trillion cycles
    } else {
        4_000_000_000_000u64 // 2 Trillion cycles
    };
    // Define the initial settings for the new canister
    let settings = CanisterSettings {
        controllers: Some(controllers), // Ensure the current canister is a controller
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: Some(Nat::from(reserved_cycles)),
        log_visibility: Some(LogVisibility::Public),
        wasm_memory_limit: None, // use default of 3GB
    };
    // Step 1: Create the canister
    let canister_id = match retry_async(
        || {
            create_canister(
                CreateCanisterArgument {
                    settings: Some(settings.clone()),
                },
                initial_cycles as u128,
            )
        },
        3,
    )
    .await
    {
        Ok(canister) => canister.0.canister_id,
        Err(e) => {
            return Err(NewArchiveError::CreateCanisterError(format!("{e:?}")));
        }
    };
    let mut current_auth_prins = read_state(|s| s.data.authorized_principals.clone());
    let test_mode = read_state(|s| s.env.is_test_mode());
    let commit_hash = read_state(|s| s.env.commit_hash().to_string());
    current_auth_prins.push(this_canister_id);

    let init_args = match Encode!(&ArgsArchive::Init(InitArgs {
        commit_hash,
        authorized_principals: current_auth_prins,
        test_mode,
    })) {
        Ok(encoded_init_args) => encoded_init_args,
        Err(e) => {
            return Err(NewArchiveError::FailedToSerializeInitArgs(format!("{e}")));
        }
    };

    // Step 2: Install the Wasm module to the newly created canister
    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: ARCHIVE_WASM.to_vec(),
        arg: init_args,
    };

    match retry_async(|| install_code(install_args.clone()), 3).await {
        Ok(_) => {
            mutate_state(|s| {
                s.data.archive_system.new_archive_error = None;
            });
            Ok(canister_id)
        }
        Err(e) => Err(NewArchiveError::InstallCodeError(format!("{e:?}"))),
    }
}

pub async fn is_archive_canister_at_threshold(archive: &ArchiveCanister) -> bool {
    let res = retry_async(|| get_archive_size(archive.canister_id, &()), 3).await;
    let max_canister_archive_threshold =
        read_state(|s| s.data.archive_system.max_canister_archive_threshold);

    match res {
        Ok(size) => (size as u128) >= max_canister_archive_threshold,
        Err(_) => false,
    }
}

pub async fn update_archive_canisters() -> Result<(), Vec<String>> {
    let archive_canisters = read_state(|s| s.data.archive_system.get_archive_canisters());
    let commit_hash = read_state(|s| s.env.commit_hash().to_string());
    let version = read_state(|s| s.env.version());
    let mut current_auth_prins = read_state(|s| s.data.authorized_principals.clone());
    let this_canister_id = read_state(|s| s.env.canister_id());
    current_auth_prins.push(this_canister_id);

    let init_args = match Encode!(&ArgsArchive::Upgrade(UpgradeArgs {
        commit_hash,
        version,
    })) {
        Ok(encoded_init_args) => encoded_init_args,
        Err(e) => {
            return Err(vec![format!(
                "ERROR : failed to create init args with error - {e}"
            )]);
        }
    };

    let mut canister_upgrade_errors = vec![];

    for archive in archive_canisters {
        match retry_async(
            || {
                stop_canister(CanisterIdRecord {
                    canister_id: archive.canister_id,
                })
            },
            3,
        )
        .await
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
                match retry_async(
                    || {
                        start_canister(CanisterIdRecord {
                            canister_id: archive.canister_id,
                        })
                    },
                    3,
                )
                .await
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

    if canister_upgrade_errors.is_empty() {
        Ok(())
    } else {
        Err(canister_upgrade_errors)
    }
}

async fn get_canister_controllers(
    canister_id: CanisterId,
) -> Result<Vec<Principal>, NewArchiveError> {
    match retry_async(|| canister_status(CanisterIdRecord { canister_id }), 3).await {
        Ok(res) => Ok(res.0.settings.controllers),
        Err(e) => Err(NewArchiveError::CantFindControllers(format!("{e:?}"))),
    }
}

pub async fn archive_stake_position(
    id: StakePositionId,
    position: StakePosition,
) -> Result<StakePositionId, ()> {
    let archive_canister = match read_state(|s| s.data.archive_system.get_active_archive_canister())
    {
        Some(canister) => canister,
        None => {
            return Err(());
        }
    };

    let args: ArchiveItemArgs = (id, position);
    match archive_item(archive_canister.canister_id, &args).await {
        Ok(()) => {
            mutate_state(|s| s.data.stake_system.remove_stake_position(id));
            debug!("position archived :: ID {id:?}");
            trace(&format!("position archived :: ID {id:?}"));
            Ok(id)
        }
        Err(e) => {
            debug!("{e:?}");
            Err(())
        }
    }
}
