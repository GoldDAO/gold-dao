use crate::lifecycle::tasks::TaskType;
use crate::numeric::{GoldPrice, GLDT, USDG};
use crate::transfer::{PendingTransfer, TransferId, Unit};
use crate::vault::{FeeBucket, Vault, VaultId};
use crate::{
    ALPHA_FACTOR, DEFAULT_GOLD_PRICE, DEFAULT_MEDIUM_RATE, MAXIUM_INTEREST_RATE,
    MINIMUM_COLLATERAL_RATIO, MINIMUM_INTEREST_RATE,
};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use usdg_minter_api::lifecycle::InitArgument;
use usdg_minter_api::VaultError;

#[cfg(test)]
pub mod tests;

thread_local! {
    static __STATE: RefCell<Option<State>> = RefCell::default();
}

pub struct State {
    pub next_vault_id: VaultId,
    pub next_transfer_id: TransferId,

    // Vault related fields
    pub fee_bucket_to_vault_ids: BTreeMap<FeeBucket, BTreeSet<VaultId>>,
    pub account_to_vault_ids: BTreeMap<Account, BTreeSet<VaultId>>,

    pub vault_id_to_vault: BTreeMap<VaultId, Vault>,

    // Liquidation pool
    pub liquidation_pool: BTreeMap<Account, USDG>,
    pub liquidation_return: BTreeMap<Account, GLDT>,

    // Pending transfers to be processed
    pub pending_transfers: BTreeMap<TransferId, PendingTransfer>,

    // 0.01g of gold price in USD
    pub one_centigram_of_gold_price: GoldPrice,

    // Medium Rate, governed by GOLDGov
    pub interest_rates: BTreeMap<FeeBucket, f64>,
    pub previous_medium_rate: f64,

    // Canister ids
    pub usdg_ledger_id: Principal,
    pub gldt_ledger_id: Principal,
    pub gold_dao_governance_id: Principal,
    pub xrc_id: Principal,

    /// Per-principal guard for all endpoints
    pub principal_guards: BTreeSet<Principal>,
    /// Guards preventing concurrent execution timer tasks
    pub active_tasks: HashSet<TaskType>,
}

impl State {
    pub fn new(init_arg: InitArgument) -> State {
        State {
            next_vault_id: 0,
            next_transfer_id: 0,
            fee_bucket_to_vault_ids: Default::default(),
            account_to_vault_ids: Default::default(),
            vault_id_to_vault: Default::default(),
            liquidation_pool: Default::default(),
            liquidation_return: Default::default(),
            pending_transfers: Default::default(),
            one_centigram_of_gold_price: DEFAULT_GOLD_PRICE,
            interest_rates: BTreeMap::from([
                (FeeBucket::Low, DEFAULT_MEDIUM_RATE),
                (FeeBucket::Medium, DEFAULT_MEDIUM_RATE),
                (FeeBucket::High, DEFAULT_MEDIUM_RATE),
            ]),
            previous_medium_rate: DEFAULT_MEDIUM_RATE,
            usdg_ledger_id: init_arg.usdg_ledger_id,
            gldt_ledger_id: init_arg.gldt_ledger_id,
            gold_dao_governance_id: init_arg.gold_dao_governance_id,
            xrc_id: init_arg.xrc_id,
            principal_guards: Default::default(),
            active_tasks: Default::default(),
        }
    }

    pub fn sum_usdg_by_fee_bucket(&self, bucket: FeeBucket) -> USDG {
        self.fee_bucket_to_vault_ids
            .get(&bucket)
            .map(|set| {
                set.iter()
                    .map(|id| self.vault_id_to_vault.get(id).unwrap().borrowed_amount)
                    .sum()
            })
            .unwrap_or(USDG::ZERO)
    }

    pub fn get_pull_factor(&self) -> f64 {
        let numerator = self.sum_usdg_by_fee_bucket(FeeBucket::High).0 as f64
            - self.sum_usdg_by_fee_bucket(FeeBucket::Low).0 as f64;
        let denominator = self
            .sum_usdg_by_fee_bucket(FeeBucket::Low)
            .checked_add(self.sum_usdg_by_fee_bucket(FeeBucket::Medium))
            .unwrap()
            .checked_add(self.sum_usdg_by_fee_bucket(FeeBucket::High))
            .unwrap();
        assert!(denominator > USDG::ZERO);
        numerator / denominator.0 as f64
    }

    pub fn update_interest_rate(&mut self) {
        let previous_medium_rate = self.previous_medium_rate;
        let medium_rate = *self.interest_rates.get(&FeeBucket::Medium).unwrap();
        let previous_low_rate = self.interest_rates.get(&FeeBucket::Low).unwrap();
        let previous_high_rate = self.interest_rates.get(&FeeBucket::High).unwrap();
        let pull_factor = self.get_pull_factor();
        if pull_factor == 0.0 {
            return;
        }
        if pull_factor > 0.0 {
            let new_high_rate = medium_rate
                * (1.0 + (previous_high_rate - previous_medium_rate) / previous_medium_rate)
                * (1.0
                    + ALPHA_FACTOR * pull_factor * (MAXIUM_INTEREST_RATE - previous_high_rate)
                        / MAXIUM_INTEREST_RATE);
            let new_low_rate = medium_rate
                * (1.0 - (previous_medium_rate - previous_low_rate) / previous_medium_rate)
                * (1.0
                    + ALPHA_FACTOR * pull_factor * (previous_medium_rate - previous_low_rate)
                        / previous_medium_rate);
            self.interest_rates.insert(
                FeeBucket::Low,
                new_low_rate.max(MINIMUM_INTEREST_RATE).min(medium_rate),
            );
            self.interest_rates.insert(
                FeeBucket::High,
                new_high_rate.min(MAXIUM_INTEREST_RATE).max(medium_rate),
            );
        } else {
            let new_high_rate = medium_rate
                * (1.0 + (previous_high_rate - previous_medium_rate) / previous_medium_rate)
                * (1.0
                    + ALPHA_FACTOR * pull_factor * (previous_high_rate - previous_medium_rate)
                        / previous_medium_rate);
            let new_low_rate = medium_rate
                * (1.0 - (previous_medium_rate - previous_low_rate) / previous_medium_rate)
                * (1.0
                    + ALPHA_FACTOR * pull_factor * (previous_low_rate - MINIMUM_INTEREST_RATE)
                        / previous_low_rate);
            self.interest_rates.insert(
                FeeBucket::Low,
                new_low_rate.max(MINIMUM_INTEREST_RATE).min(medium_rate),
            );
            self.interest_rates.insert(
                FeeBucket::High,
                new_high_rate.min(MAXIUM_INTEREST_RATE).max(medium_rate),
            );
        }
        self.previous_medium_rate = medium_rate;
    }

    pub fn charge_fee(&mut self) {
        for id in self
            .vault_id_to_vault
            .keys()
            .cloned()
            .collect::<Vec<VaultId>>()
        {
            self.charge_fee_on_vault(id);
        }
    }

    fn charge_fee_on_vault(&mut self, vault_id: VaultId) {
        let vault = self.get_vault(vault_id).unwrap();
        let interest_rate = self.interest_rates.get(&vault.fee_bucket).unwrap();
        let daily_fee = interest_rate / 365.0;
        let fee_amount = vault.borrowed_amount.0 as f64 * daily_fee;
        let fee_e8s = USDG::from_e8s(fee_amount as u64);
        let new_borrowed_amount = vault.borrowed_amount.checked_add(fee_e8s).unwrap();
        match self.vault_id_to_vault.get_mut(&vault_id) {
            Some(vault_mut) => {
                vault_mut.borrowed_amount = new_borrowed_amount;
            }
            None => panic!("attempted to modify unkown vault"),
        };
    }

    pub fn check_max_borrowable_amount(
        &self,
        gldt_margin: GLDT,
        usdg_borrowed: USDG,
    ) -> Result<(), VaultError> {
        let max_borrowable_amount = gldt_margin
            .checked_mul_rate(self.one_centigram_of_gold_price)
            .unwrap()
            .checked_div_factor(MINIMUM_COLLATERAL_RATIO)
            .unwrap();

        if usdg_borrowed > max_borrowable_amount {
            return Err(VaultError::BorrowedAmountTooBig {
                maximum_borrowable_amount: max_borrowable_amount.0,
            });
        }

        Ok(())
    }

    pub fn get_vaults_by_account(&self, account: impl Into<Account>) -> Vec<Vault> {
        self.account_to_vault_ids
            .get(&account.into())
            .unwrap_or(&Default::default())
            .iter()
            .map(|vault| self.vault_id_to_vault.get(vault).unwrap())
            .cloned()
            .collect()
    }

    pub fn get_vault(&self, vault_id: u64) -> Option<Vault> {
        self.vault_id_to_vault.get(&vault_id).cloned()
    }

    pub fn increment_vault_id(&mut self) -> u64 {
        let vault_id = self.next_vault_id;
        self.next_vault_id += 1;
        vault_id
    }

    pub fn increment_transfer_id(&mut self) -> u64 {
        let transfer_id = self.next_transfer_id;
        self.next_transfer_id += 1;
        transfer_id
    }

    pub fn record_vault_creation(
        &mut self,
        owner: Account,
        borrowed_amount: USDG,
        margin_amount: GLDT,
        fee_bucket: impl Into<FeeBucket>,
    ) -> VaultId {
        self.check_max_borrowable_amount(margin_amount, borrowed_amount)
            .unwrap();
        let fee_bucket: FeeBucket = fee_bucket.into();
        let vault_id = self.increment_vault_id();
        let new_vault = Vault {
            vault_id,
            owner,
            borrowed_amount,
            margin_amount,
            fee_bucket,
        };
        assert!(self.vault_id_to_vault.insert(vault_id, new_vault).is_none());
        assert!(self
            .fee_bucket_to_vault_ids
            .entry(fee_bucket)
            .or_default()
            .insert(vault_id));
        assert!(self
            .account_to_vault_ids
            .entry(owner)
            .or_default()
            .insert(vault_id));
        if borrowed_amount > USDG::ZERO {
            let transfer_id = self.increment_transfer_id();
            assert!(self
                .pending_transfers
                .insert(
                    transfer_id,
                    PendingTransfer {
                        transfer_id,
                        amount: borrowed_amount.0,
                        receiver: owner,
                        unit: Unit::USDG,
                    }
                )
                .is_none());
        }
        vault_id
    }

    pub fn record_borrow_from_vault(&mut self, vault_id: VaultId, borrowed_amount: USDG) {
        let vault = self.get_vault(vault_id).unwrap();
        let new_borrowed_amount = vault.borrowed_amount.checked_add(borrowed_amount).unwrap();
        self.check_max_borrowable_amount(vault.margin_amount, new_borrowed_amount)
            .unwrap();
        match self.vault_id_to_vault.get_mut(&vault_id) {
            Some(vault_mut) => {
                vault_mut.borrowed_amount = new_borrowed_amount;
            }
            None => panic!("attempted to borrow from unkown vault"),
        };
    }

    pub fn record_add_margin_to_vault(&mut self, vault_id: VaultId, margin_amount: GLDT) {
        match self.vault_id_to_vault.get_mut(&vault_id) {
            Some(vault) => {
                vault.margin_amount = vault.margin_amount.checked_add(margin_amount).unwrap();
            }
            None => panic!("attempted to add maring to unkown vault"),
        };
    }

    pub fn deposit_liquidity(&mut self, to: Account, amount: USDG) {
        self.liquidation_pool
            .entry(to)
            .and_modify(|balance| *balance = balance.checked_add(amount).unwrap())
            .or_insert(amount);
    }

    pub fn withdraw_liquidity(&mut self, amount: USDG, from: Account) {
        match self.liquidation_pool.entry(from) {
            Occupied(mut entry) => {
                *entry.get_mut() = entry.get().checked_sub(amount).unwrap();
                if *entry.get() == USDG::ZERO {
                    entry.remove_entry();
                }
            }
            Vacant(_) => ic_cdk::trap("cannot remove liquidity from unknow principal"),
        }
    }
}

pub fn mutate_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    __STATE.with(|s| f(s.borrow_mut().as_mut().expect("State not initialized!")))
}

pub fn read_state<F, R>(f: F) -> R
where
    F: FnOnce(&State) -> R,
{
    __STATE.with(|s| f(s.borrow().as_ref().expect("State not initialized!")))
}

pub fn replace_state(state: State) {
    __STATE.with(|s| {
        *s.borrow_mut() = Some(state);
    });
}
