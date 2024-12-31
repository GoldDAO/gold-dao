use crate::logs::INFO;
use crate::numeric::{Factor, GoldPrice, GLDT, USDG};
use crate::state::State;
use crate::MINIMUM_COLLATERAL_RATIO;
use candid::CandidType;
use ic_canister_log::log;
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;
use std::fmt;
use usdg_minter_api::ApiFeeBucket;

pub type VaultId = u64;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vault {
    // The vault unique Id.
    pub vault_id: VaultId,
    // The owner of the vault.
    pub owner: Account,
    // The amount of USDG currently borrowed from this vault.
    // Represented using e8s.
    pub borrowed_amount: USDG,
    // The GLDT margin of this vault.
    // Represented using e8s.
    pub margin_amount: GLDT,
    // The bucket which determines the fee charged over time.
    pub fee_bucket: FeeBucket,
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vault {{ id: {}, owner: {}, borrowed: {} USDG, margin: {} GLDT, fee bucket: {} }}",
            self.vault_id, self.owner, self.borrowed_amount, self.margin_amount, self.fee_bucket
        )
    }
}

impl Vault {
    pub fn compute_collateral_ratio(&self, gold_price: GoldPrice) -> Factor {
        if self.borrowed_amount == USDG::ZERO {
            return Factor::MAX;
        }
        let margin_value: USDG = self.margin_amount.checked_mul_rate(gold_price).unwrap();
        margin_value.checked_div(self.borrowed_amount).unwrap()
    }
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeeBucket {
    Low = 0,
    Medium = 1,
    High = 2,
}

impl fmt::Display for FeeBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeeBucket::Low => {
                write!(f, "Low",)
            }
            FeeBucket::Medium => {
                write!(f, "Medium",)
            }
            FeeBucket::High => {
                write!(f, "High",)
            }
        }
    }
}

impl From<ApiFeeBucket> for FeeBucket {
    fn from(api_bucket: ApiFeeBucket) -> FeeBucket {
        match api_bucket {
            ApiFeeBucket::Low => FeeBucket::Low,
            ApiFeeBucket::Medium => FeeBucket::Medium,
            ApiFeeBucket::High => FeeBucket::High,
        }
    }
}

pub fn check_vaults(state: &mut State) {
    let last_gold_price = state.one_centigram_of_gold_price;
    let (unhealthy_vaults, has_healthy_vault) = {
        let mut unhealthy_vaults: Vec<(VaultId, USDG)> = vec![];
        let mut has_healthy_vault = false;
        for vault in state.vault_id_to_vault.values() {
            if vault.compute_collateral_ratio(last_gold_price) < MINIMUM_COLLATERAL_RATIO {
                unhealthy_vaults.push((vault.vault_id, vault.borrowed_amount));
            } else {
                has_healthy_vault = true;
            }
        }
        (unhealthy_vaults, has_healthy_vault)
    };
    for (vault_id, borrowed_amount) in unhealthy_vaults {
        let provided_liquidity = state.liquidation_pool.values().cloned().sum();
        if borrowed_amount <= provided_liquidity {
            log!(
                INFO,
                "[check_vaults] liquidate vault {vault_id} to liquidity pool with liquidity: {provided_liquidity} USDG",
            );
            // TODO this should be recorded as an event
            state.record_liquidate_vault_liquidation_pool(vault_id);
            // process_event(
            //     s,
            //     EventType::LiquidateVault {
            //         vault_id: vault.vault_id,
            //     },
            // )
        } else if has_healthy_vault {
            log!(
                INFO,
                "[check_vaults] redistribute vault {vault_id} to all the other vaults.",
            );
            state.record_redistribute_vault(vault_id);
            // mutate_state(|s| {
            //     process_event(
            //         s,
            //         EventType::RedistributeVault {
            //             vault_id: vault.vault_id,
            //         },
            //     )
            // });
        }
    }
}

pub fn get_redemption_fee(redeemed_amount: USDG, total_borrowed: USDG) -> Factor {
    const ONE_HALF: Factor = Factor::from_e8s(50_000_000);
    if total_borrowed == USDG::ZERO {
        return Factor::from_e8s(500_000);
    }
    redeemed_amount
        .checked_div(total_borrowed)
        .unwrap()
        .checked_mul(ONE_HALF)
        .unwrap()
        .max(Factor::from_e8s(500_000))
        .min(Factor::from_e8s(5_000_000))
}
