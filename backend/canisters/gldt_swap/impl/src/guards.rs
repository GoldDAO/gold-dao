use crate::state::mutate_state;
use crate::state::read_state;
use candid::Principal;
use gldt_swap_common::nft::Nft;
use std::marker::PhantomData;

// TODO: think on the limit, how much NFTs are issued now? + how much can be issued in the future?
const MAX_CONCURRENT: usize = 100;

/// Guards a block from executing twice when called by the same user and from being
/// executed [MAX_CONCURRENT] or more times in parallel.
#[must_use]
pub struct GuardNft {
    nft: Nft,
    _marker: PhantomData<GuardNft>,
}

impl GuardNft {
    /// Attempts to create a new guard for the current block. Fails if there is
    /// already a pending request for the specified [nft] or if there
    /// are at least [MAX_CONCURRENT] pending requests.
    pub fn new(nft: Nft) -> Result<Self, String> {
        mutate_state(|s| {
            if s.data.nft_guards.contains(&nft) {
                return Err("Error: Duplicate request".to_string());
            }
            if s.data.nft_guards.len() >= MAX_CONCURRENT {
                return Err("Service is too busy, try again shortly".to_string());
            }
            s.data.nft_guards.insert(nft.clone());
            Ok(Self {
                nft,
                _marker: PhantomData,
            })
        })
    }
}

impl Drop for GuardNft {
    fn drop(&mut self) {
        mutate_state(|s| s.data.nft_guards.remove(&self.nft));
    }
}

pub fn caller_is_nft_canister() -> Result<(), String> {
    if read_state(|state| state.is_caller_is_nft_canister()) {
        Ok(())
    } else {
        Err("Caller is not a valid NFT canister".to_string())
    }
}

pub fn caller_is_authorized() -> Result<(), String> {
    if read_state(|state| state.is_caller_authorized()) {
        Ok(())
    } else {
        Err("Caller is not an authorized principal".to_string())
    }
}

pub fn reject_anonymous_caller() -> Result<(), String> {
    if ic_cdk::api::msg_caller() == Principal::anonymous() {
        return Err("You may not use an anonymous principal".to_string());
    }
    Ok(())
}
