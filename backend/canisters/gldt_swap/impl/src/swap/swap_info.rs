use std::future::Future;

use gldt_swap_common::swap::{ SwapId, SwapInfo, SwapStatus, SwapStatusForward, SwapStatusReverse };
use gldt_swap_api_archive::archive_swap::Args as ArchiveSwapArg;
use gldt_swap_archive_c2c_client::archive_swap;
use tracing::{ debug, info };

use crate::{
    check_storage_and_create_archive,
    state::{ mutate_state, read_state },
    utils::{ get_historic_swap, trace },
};

pub trait SwapInfoTrait {
    fn insert_swap(&self) -> impl Future<Output = Result<SwapId, ()>> + Send;
    fn update_status(&self, status: SwapStatus) -> ();
    fn set_recovery_mode(&self, new_state: bool) -> ();
    fn move_swap_to_history(&self) -> impl Future<Output = Result<SwapId, ()>> + Send;
}

impl SwapInfoTrait for SwapInfo {
    async fn insert_swap(&self) -> Result<SwapId, ()> {
        // check if it already exists - can't insert a swap that already exists
        let nft_id = match &self {
            SwapInfo::Forward(deets) => deets.nft_id.clone(),
            SwapInfo::Reverse(deets) => deets.nft_id.clone(),
        };

        if read_state(|s| s.data.swaps.is_nft_locked(&nft_id)) {
            debug!("Swap is already present");
            return Err(());
        }

        if check_storage_and_create_archive().await.is_err() {
            return Err(());
            // TODO - return error code here
        }
        let current_index = read_state(|s| s.data.swaps.get_current_swap_index());
        mutate_state(|s| s.data.swaps.increment_swap_index());

        // use the latest index
        let mut new_swap = self.clone();
        match &mut new_swap {
            SwapInfo::Forward(deets) => {
                deets.index = current_index;
            }
            SwapInfo::Reverse(deets) => {
                deets.index = current_index;
            }
        }

        // insert to active or history depending on the status
        match new_swap.get_status() {
            | SwapStatus::Forward(SwapStatusForward::Init)
            | SwapStatus::Reverse(SwapStatusReverse::Init) => {
                if
                    let Ok(swap_id) = mutate_state(|s|
                        s.data.swaps.insert_active_swap(&nft_id.clone(), &new_swap.clone())
                    )
                {
                    Ok(swap_id)
                } else {
                    Err(())
                }
            }
            | SwapStatus::Forward(SwapStatusForward::Failed(_))
            | SwapStatus::Reverse(SwapStatusReverse::Failed(_)) => {
                if let Ok(swap_id) = new_swap.move_swap_to_history().await {
                    Ok(swap_id)
                } else {
                    Err(())
                }
            }
            _ => { Err(()) }
        }
    }

    fn update_status(&self, status: SwapStatus) {
        let swap_id = self.get_swap_id();

        mutate_state(|s| {
            let mut swap = s.data.swaps.get_active_swap_mut(&swap_id);
            match &status {
                SwapStatus::Forward(new_status) => {
                    if let Some(SwapInfo::Forward(swap_detail)) = &mut swap {
                        swap_detail.status = new_status.clone();
                        info!(
                            "FORWARD SWAP :: SwapId {swap_id:?} :: status updated -> {new_status:?} :: swap detail -> {swap_detail:?}"
                        );
                    }
                }
                SwapStatus::Reverse(new_status) => {
                    if let Some(SwapInfo::Reverse(swap_detail)) = &mut swap {
                        swap_detail.status = new_status.clone();
                        info!(
                            "REVERSE SWAP :: SwapId {swap_id:?} :: status updated -> {new_status:?} :: swap detail -> {swap_detail:?}"
                        );
                    }
                }
            }
        });

        let should_move_to_history = matches!(
            status,
            SwapStatus::Forward(SwapStatusForward::Complete) |
                SwapStatus::Forward(SwapStatusForward::Failed(_)) |
                SwapStatus::Reverse(SwapStatusReverse::Complete) |
                SwapStatus::Reverse(SwapStatusReverse::Failed(_))
        );

        if should_move_to_history {
            if let Some(swap) = read_state(|s| s.data.swaps.get_active_swap(&swap_id).cloned()) {
                ic_cdk::spawn(async move {
                    let _ = swap.move_swap_to_history().await;
                });
            }
        }
    }

    fn set_recovery_mode(&self, new_state: bool) {
        let swap_id = self.get_swap_id();

        mutate_state(|s| {
            let mut swap = s.data.swaps.get_active_swap_mut(&swap_id);
            match swap {
                Some(swap) => {
                    match swap {
                        SwapInfo::Forward(_) => {}
                        SwapInfo::Reverse(swap_detail_reverse) => {
                            swap_detail_reverse.in_recovery_mode = new_state;
                        }
                    }
                }
                None => {
                    debug!(
                        "ERROR :: swap with id {swap_id:?} can't be toggled to recovery mode because the swap can't be found"
                    );
                }
            }
        });
    }

    async fn move_swap_to_history(&self) -> Result<SwapId, ()> {
        let swap_id = self.get_swap_id();
        let archive_canister = match
            read_state(|s| s.data.swaps.find_canister_for_swap_index(swap_id.1.clone()))
        {
            Some(canister_id) => canister_id,
            None => {
                return Err(());
            }
        };

        if get_historic_swap(&swap_id).await.is_some() {
            let message = format!(
                "ERROR : can't insert swap with SwapId : {swap_id:?}. it already exists in history"
            );
            debug!(message);
            trace(&message);
            return Err(());
        }

        let args: ArchiveSwapArg = (swap_id.clone(), self.clone());
        match archive_swap(archive_canister.clone(), &args).await {
            Ok(()) => {
                mutate_state(|s| s.data.swaps.remove_swap_from_active_swaps(&swap_id));
                debug!(
                    "SWAP FINISHED :: SwapId {swap_id:?} with status {:?}:: swap moved to history",
                    self.get_status()
                );

                Ok(swap_id)
            }
            Err(e) => {
                debug!("{e:?}");
                Err(())
            }
        }
    }
}
