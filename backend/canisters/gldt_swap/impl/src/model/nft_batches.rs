use crate::state::read_state;
use candid::{Nat, Principal};
use futures::future::join_all;
use gldt_swap_common::general_error::GeneralError;
use gldt_swap_common::nft::Nft;
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_canister::icrc37_transfer_from::{
    TransferFromArg, TransferFromError, TransferFromResult,
};
use origyn_nft_canister::icrc7_transfer::TransferError;
use origyn_nft_canister::TransferArg;
use origyn_nft_canister_c2c_client::{icrc37_transfer_from, icrc7_owner_of, icrc7_transfer};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tracing::{error, info, warn};
use types::CanisterId;

#[derive(Debug, Default)]
pub struct GroupedNftsByCanister {
    pub grouped: HashMap<CanisterId, Vec<Nat>>,
}

impl GroupedNftsByCanister {
    pub fn from_nfts(nfts: impl IntoIterator<Item = Nft>) -> Self {
        let mut grouped: HashMap<Principal, Vec<Nat>> = HashMap::new();

        for nft in nfts {
            grouped.entry(nft.canister_id).or_default().push(nft.id);
        }

        Self { grouped }
    }

    pub fn add_nft(&mut self, canister_id: CanisterId, nft_id: Nat) {
        self.grouped.entry(canister_id).or_default().push(nft_id);
    }

    pub fn exclude_nft(mut self, nft: Nft) -> Self {
        if let Some(ids) = self.grouped.get_mut(&nft.canister_id) {
            ids.retain(|id| id != &nft.id);

            // Remove the entire canister entry if it has no NFTs left
            if ids.is_empty() {
                self.grouped.remove(&nft.canister_id);
            }
        }
        self
    }

    pub async fn validate_all_owned_by(&self, user: Principal) -> Vec<Result<Nft, GeneralError>> {
        let mut results = Vec::new();

        for (canister_id, ids) in &self.grouped {
            match icrc7_owner_of(*canister_id, ids.clone()).await {
                Ok(owners) => {
                    for (idx, owner_opt) in owners.iter().enumerate() {
                        let nft = Nft {
                            canister_id: *canister_id,
                            id: ids[idx].clone(),
                        };

                        match owner_opt {
                            Some(owner) if *owner == user.into() => {
                                results.push(Ok(nft));
                            }
                            _ => {
                                results.push(Err(GeneralError::UserIsNotNftOwner(format!(
                                    "User {:?} is not owner of the nft {:?} from the cansiter {:?}",
                                    user, ids[idx], canister_id
                                ))));
                            }
                        }
                    }
                }
                Err(e) => {
                    for id in ids {
                        results.push(Err(GeneralError::CallError(format!(
                            "Canister {canister_id:?}, NFT {id:?}: {e:?}"
                        ))));
                    }
                }
            }
        }

        results
    }

    pub async fn batch_transfer_from_all(
        &self,
        from: Principal,
    ) -> BTreeMap<Principal, Vec<Result<Nat, TransferFromError>>> {
        // Step 1: collect metadata and futures separately
        let mut metadata: Vec<(Principal, Vec<Nat>)> = Vec::new();
        let mut futures: Vec<_> = Vec::new();

        for (canister_id, nft_ids) in &self.grouped {
            info!(
            "Preparing transfer for {} NFTs from {:?} to self canister {:?} (target canister_id={})",
            nft_ids.len(),
            from,
            ic_cdk::api::canister_self(),
            canister_id
        );

            let transfers: Vec<TransferFromArg> = nft_ids
                .iter()
                .map(|token_id| TransferFromArg {
                    spender_subaccount: None,
                    from: Account {
                        owner: from,
                        subaccount: None,
                    },
                    to: ic_cdk::api::canister_self().into(),
                    token_id: token_id.clone(),
                    memo: None,
                    created_at_time: None,
                })
                .collect();

            metadata.push((*canister_id, nft_ids.clone()));
            futures.push(icrc37_transfer_from(*canister_id, transfers));
        }

        info!(
            "Executing {} batch transfer futures in parallel",
            futures.len()
        );

        // Step 2: await all futures in parallel
        let transfer_results: Vec<_> = join_all(futures).await;

        // Step 3: zip metadata and results back together
        let mut all_results: BTreeMap<Principal, Vec<Result<Nat, TransferFromError>>> =
            BTreeMap::new();

        for ((canister_id, nft_ids), call_result) in metadata.into_iter().zip(transfer_results) {
            info!(
                "Processing transfer result for canister_id={} ({} NFTs)",
                canister_id,
                nft_ids.len()
            );

            let results: Vec<Result<Nat, TransferFromError>> = match call_result {
                // Case: The canister call succeeded and returned individual results
                Ok(Ok(vec_of_opts)) => vec_of_opts
                    .into_iter()
                    .enumerate()
                    .map(|(idx, maybe_res)| match maybe_res {
                        Some(TransferFromResult::Ok(_amount)) => {
                            info!(
                                "NFT {:?} successfully transferred from {:?}",
                                nft_ids[idx], from
                            );
                            Ok(nft_ids[idx].clone())
                        }
                        Some(TransferFromResult::Err(e)) => {
                            error!("NFT {:?} transfer error: {:?}", nft_ids[idx], e);
                            Err(e)
                        }
                        None => {
                            warn!("NFT {:?} missing transfer response", nft_ids[idx]);
                            Err(TransferFromError::GenericError {
                                error_code: Nat::from(0_u64),
                                message: "No response from transfer".to_string(),
                            })
                        }
                    })
                    .collect(),

                // Case: The canister call succeeded but returned a batch-level error
                Ok(Err(batch_err)) => nft_ids.iter().map(|_| Err(batch_err.clone())).collect(),

                // Case: The inter-canister call itself failed (network / candid / system error)
                Err(e) => {
                    let msg = format!("{:?}", e);
                    error!(
                        "Inter-canister call failed for canister {}: {}",
                        canister_id, msg
                    );
                    nft_ids
                        .iter()
                        .map(|_| {
                            Err(TransferFromError::GenericError {
                                error_code: Nat::from(0_u64),
                                message: format!("Batch transfer call failed: {}", msg),
                            })
                        })
                        .collect()
                }
            };

            all_results.insert(canister_id, results);
        }

        info!(
            "Batch transfer finished for {} canisters",
            all_results.len()
        );

        all_results
    }

    pub async fn batch_transfer_all(
        &self,
        to: Principal,
    ) -> BTreeMap<Principal, Vec<Result<Nat, TransferError>>> {
        let mut all_results: BTreeMap<Principal, Vec<Result<Nat, TransferError>>> = BTreeMap::new();

        for (canister_id, nft_ids) in &self.grouped {
            let transfers: Vec<TransferArg> = nft_ids
                .iter()
                .map(|token_id| TransferArg {
                    to: to.into(),
                    token_id: token_id.clone(),
                    from_subaccount: None,
                    memo: None,
                    created_at_time: None,
                })
                .collect();

            let results: Vec<Result<Nat, TransferError>> =
                match icrc7_transfer(*canister_id, transfers).await {
                    Ok(response_vec) => response_vec
                        .into_iter()
                        .enumerate()
                        .map(|(idx, resp)| match resp {
                            Some(Ok(_)) => {
                                // Always return the ID we requested, not the ID the canister echoes
                                Ok(nft_ids[idx].clone())
                            }
                            Some(Err(e)) => Err(e),
                            None => Err(TransferError::GenericError {
                                error_code: Nat::from(0_u64),
                                message: "No response from transfer".to_string(),
                            }),
                        })
                        .collect(),
                    Err(e) => nft_ids
                        .iter()
                        .map(|_| {
                            Err(TransferError::GenericError {
                                error_code: Nat::from(0_u64),
                                message: format!("Batch transfer failed: {}", e),
                            })
                        })
                        .collect(),
                };

            all_results.insert(*canister_id, results);
        }

        all_results
    }

    pub fn calculate_token_equivalent(&self) -> Result<HashMap<Principal, Nat>, GeneralError> {
        let swap_configs = read_state(|s| s.data.swap_configs.clone());

        let mut ledger_totals: HashMap<Principal, Nat> = HashMap::new();

        for (canister_id, token_ids) in &self.grouped {
            let config = swap_configs.find_by_canister_id(canister_id)?;
            for token_id in token_ids {
                let amount = config
                    .fractionalization_config
                    .division_for_token(token_id)?;
                let ledger_id = config.fractionalization_config.get_ledger_id()?;
                *ledger_totals.entry(ledger_id).or_insert(Nat::from(0_u64)) += amount;
            }
        }

        Ok(ledger_totals)
    }

    pub fn calculate_fees_amount(&self) -> Result<HashMap<Principal, Nat>, GeneralError> {
        let swap_configs = read_state(|s| s.data.swap_configs.clone());

        let mut fee_totals: HashMap<Principal, Nat> = HashMap::new();

        for (canister_id, token_ids) in &self.grouped {
            let config = swap_configs.find_by_canister_id(canister_id)?;

            for token_id in token_ids {
                // Get the swap fee for this token
                let fee = config
                    .fractionalization_config
                    .swap_fee_for_token(token_id)?;

                // Ledger ID
                let ledger_id = config.fractionalization_config.get_ledger_id()?;

                *fee_totals.entry(ledger_id).or_insert(Nat::from(0_u64)) += fee;
            }
        }

        Ok(fee_totals)
    }

    // Removes all NFTs that appear in the provided guard set.
    pub fn filter(mut self, nft_guards: &BTreeSet<Nft>) -> Self {
        for (canister_id, ids) in self.grouped.iter_mut() {
            ids.retain(|id| {
                let nft = Nft {
                    canister_id: *canister_id,
                    id: id.clone(),
                };
                !nft_guards.contains(&nft)
            });
        }

        // Clean up empty entries
        self.grouped.retain(|_, ids| !ids.is_empty());

        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use candid::{Nat, Principal};
    use gldt_swap_common::nft::Nft;
    use gldt_swap_common::swap_canister_config::SwapCanisterConfig;
    use std::collections::HashSet;

    use crate::{
        model::swap_configs::SwapConfigs,
        state::{init_state, mutate_state, RuntimeState},
    };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    fn sum_result_map(
        result: Result<HashMap<Principal, Nat>, GeneralError>,
    ) -> Result<Nat, GeneralError> {
        result.map(|map| {
            map.values()
                .fold(Nat::from(0u128), |acc, val| acc + val.clone())
        })
    }

    #[test]
    fn test_update_neuron_rewards() {
        init_runtime_state();

        let swap_config_1000 = SwapCanisterConfig {
            icrc7_canister_id: Principal::from_text("7i7jl-6qaaa-aaaam-abjma-cai").unwrap(),
            fractionalization_config:
                gldt_swap_common::swap_canister_config::FractionalizationConfig::General(
                    gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig {
                        division: 1000,
                        swap_fee: Nat::from(100_000_000_u64),
                        ledger_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
                    },
                ),
        };
        let swap_config_100 = SwapCanisterConfig {
            icrc7_canister_id: Principal::from_text("zhfjc-liaaa-aaaal-acgja-cai").unwrap(),
            fractionalization_config:
                gldt_swap_common::swap_canister_config::FractionalizationConfig::General(
                    gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig {
                        division: 100,
                        swap_fee: Nat::from(100_000_000_u64),
                        ledger_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
                    },
                ),
        };
        let swap_config_10 = SwapCanisterConfig {
            icrc7_canister_id: Principal::from_text("sy3ra-iqaaa-aaaao-aixda-cai").unwrap(),
            fractionalization_config:
                gldt_swap_common::swap_canister_config::FractionalizationConfig::General(
                    gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig {
                        division: 10,
                        swap_fee: Nat::from(100_000_000_u64),
                        ledger_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
                    },
                ),
        };
        let swap_config_1 = SwapCanisterConfig {
            icrc7_canister_id: Principal::from_text("io7gn-vyaaa-aaaak-qcbiq-cai").unwrap(),
            fractionalization_config:
                gldt_swap_common::swap_canister_config::FractionalizationConfig::General(
                    gldt_swap_common::swap_canister_config::GeneralFractionalizationConfig {
                        division: 1,
                        swap_fee: Nat::from(100_000_000_u64),
                        ledger_id: Principal::from_text("6c7su-kiaaa-aaaar-qaira-cai").unwrap(),
                    },
                ),
        };
        mutate_state(|state| {
            state.data.swap_configs = SwapConfigs::from_vec(vec![
                swap_config_1000,
                swap_config_100,
                swap_config_10,
                swap_config_1,
            ]);
        });

        let nfts: HashSet<Nft> = [
            Nft {
                canister_id: Principal::from_text("sy3ra-iqaaa-aaaao-aixda-cai").unwrap(), // 10
                id: Nat::from(4u64),
            },
            Nft {
                canister_id: Principal::from_text("zhfjc-liaaa-aaaal-acgja-cai").unwrap(), // 100
                id: Nat::from(1u64),
            },
            Nft {
                canister_id: Principal::from_text("zhfjc-liaaa-aaaal-acgja-cai").unwrap(), // 100
                id: Nat::from(2u64),
            },
            Nft {
                canister_id: Principal::from_text("io7gn-vyaaa-aaaak-qcbiq-cai").unwrap(), // 1
                id: Nat::from(101u64),
            },
        ]
        .into_iter()
        .collect();

        let nfts_batch = GroupedNftsByCanister::from_nfts(nfts);

        let totals = sum_result_map(nfts_batch.calculate_token_equivalent()).unwrap();
        assert_eq!(totals, Nat::from(211_u64));
    }
}
