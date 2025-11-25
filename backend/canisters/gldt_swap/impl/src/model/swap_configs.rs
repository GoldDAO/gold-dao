use candid::Principal;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use gldt_swap_common::swap_canister_config::{
    FractionalizationConfig, GeneralFractionalizationConfig, SwapCanisterConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::error;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SwapConfigs {
    pub configs: Vec<SwapCanisterConfig>,
}

impl SwapConfigs {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    pub fn from_vec(configs: Vec<SwapCanisterConfig>) -> Self {
        Self { configs }
    }

    pub fn find_by_canister_id(
        &self,
        canister_id: &Principal,
    ) -> Result<&SwapCanisterConfig, GeneralError> {
        self.configs
            .iter()
            .find(|cfg| cfg.icrc7_canister_id == *canister_id)
            .ok_or_else(|| {
                GeneralError::ConfigNotFound(format!(
                    "No swap config found for canister {canister_id}"
                ))
            })
    }

    pub fn unique_ledger_ids(&self) -> HashSet<Principal> {
        self.configs
            .iter()
            .filter_map(|cfg| cfg.fractionalization_config.get_ledger_id().ok())
            .collect()
    }

    pub fn get_valid_nft_canisters(&self) -> Vec<Principal> {
        self.configs
            .iter()
            .map(|config| config.icrc7_canister_id)
            .collect()
    }

    pub fn validate_nfts(&self, args: &HashSet<Nft>) -> Result<(), GeneralError> {
        let nft_canisters = self.get_valid_nft_canisters();

        if args
            .iter()
            .any(|nft| !nft_canisters.contains(&nft.canister_id))
        {
            error!("Invalid NFT canisters provided: {:?}", args);

            return Err(GeneralError::InvalidNftCanister(format!(
                "You may not specify an unknown GLD NFT canister. \
                 Check that all your intended swaps contain a valid NFT canister principal \
                 that match one of these: \n {nft_canisters:?}"
            )));
        }

        Ok(())
    }

    pub fn get_nft_equivalent(
        &self,
        nft: &Nft,
    ) -> Result<GeneralFractionalizationConfig, GeneralError> {
        let cfg = self.find_by_canister_id(&nft.canister_id)?;

        match &cfg.fractionalization_config {
            FractionalizationConfig::General(gen_cfg) => Ok(gen_cfg.clone()),
            FractionalizationConfig::Custom(custom_cfg) => custom_cfg
                .per_token_config
                .get(&nft.id)
                .cloned()
                .ok_or_else(|| {
                    GeneralError::ConfigNotFound(format!(
                        "No fractionalization config found for NFT {} in canister {}",
                        nft.id, nft.canister_id
                    ))
                }),
        }
    }

    pub fn get_token_ledger_for_nft(
        &self,
        nft_canister_id: &Principal,
    ) -> Result<Principal, GeneralError> {
        let cfg = self
            .configs
            .iter()
            .find(|cfg| cfg.icrc7_canister_id == *nft_canister_id)
            .ok_or_else(|| {
                GeneralError::ConfigNotFound(format!(
                    "No swap config found for NFT canister {}",
                    nft_canister_id
                ))
            })?;

        // Propagate the Result from get_ledger_id
        cfg.fractionalization_config.get_ledger_id().map_err(|_| {
            GeneralError::ConfigNotFound(format!(
                "No ledger associated with NFT canister {}",
                nft_canister_id
            ))
        })
    }
}
