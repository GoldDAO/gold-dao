use crate::model::nft_batches::GroupedNftsByCanister;
use crate::model::swap_configs::SwapConfigs;
use candid::Principal;
use gldt_swap_common::general_error::GeneralError;
use origyn_nft_canister::icrc7_tokens_of;
use origyn_nft_canister_c2c_client::icrc7_tokens_of;

#[async_trait::async_trait]
pub trait NftCollector {
    async fn collect_nfts(&self, user: Principal) -> Result<GroupedNftsByCanister, GeneralError>;
}

#[async_trait::async_trait]
impl NftCollector for SwapConfigs {
    async fn collect_nfts(&self, user: Principal) -> Result<GroupedNftsByCanister, GeneralError> {
        // Spawn futures for each configured canister
        let futures: Vec<_> = self
            .configs
            .iter()
            .map(|config| {
                let canister_id = config.icrc7_canister_id;
                let fut = icrc7_tokens_of(
                    canister_id,
                    (
                        icrc7_tokens_of::Args0 {
                            owner: user,
                            subaccount: None,
                        },
                        None,
                        None,
                    ),
                );
                (canister_id, fut)
            })
            .collect();

        // Run all calls in parallel
        let results: Vec<_> = futures::future::join_all(
            futures
                .into_iter()
                .map(|(canister_id, fut)| async move { (canister_id, fut.await) }),
        )
        .await;

        let mut grouped = GroupedNftsByCanister::default();

        for (canister_id, res) in results {
            match res {
                Ok((nft_ids,)) if !nft_ids.is_empty() => {
                    for nft_id in nft_ids {
                        grouped.add_nft(canister_id, nft_id);
                    }
                }
                Ok(_) => {} // no NFTs for this canister
                Err(e) => return Err(GeneralError::CallError(e.to_string())),
            }
        }

        Ok(grouped)
    }
}
