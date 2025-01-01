use crate::lifecycle::tasks::TaskType;
use crate::numeric::{Factor, GoldPrice, GLDT, USDG};
use crate::transfer::{PendingTransfer, TransferId, Unit};
use crate::vault::{get_redemption_fee, FeeBucket, Vault, VaultId};
use crate::{
    ALPHA_FACTOR, DEFAULT_GOLD_PRICE, DEFAULT_MEDIUM_RATE, MAXIUM_INTEREST_RATE,
    MINIMUM_COLLATERAL_RATIO, MINIMUM_INTEREST_RATE,
};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use usdg_minter_api::lifecycle::InitArgument;
use usdg_minter_api::VaultError;

pub mod audit;
pub mod event;
#[cfg(test)]
pub mod tests;

// Like assert_eq, but returns an error instead of panicking.
macro_rules! ensure_eq {
    ($lhs:expr, $rhs:expr, $msg:expr $(, $args:expr)* $(,)*) => {
        if $lhs != $rhs {
            return Err(format!("{} ({:?}) != {} ({:?}): {}",
                               std::stringify!($lhs), $lhs,
                               std::stringify!($rhs), $rhs,
                               format!($msg $(,$args)*)));
        }
    }
}

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

    // Reserve
    pub reserve_usdg: USDG,

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
            reserve_usdg: USDG::ZERO,
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

    pub fn total_gldt_margin(&self) -> GLDT {
        self.vault_id_to_vault
            .values()
            .fold(GLDT::ZERO, |acc, vault| {
                acc.checked_add(vault.margin_amount).unwrap()
            })
    }

    pub fn total_usdg_debt(&self) -> USDG {
        self.vault_id_to_vault
            .values()
            .fold(USDG::ZERO, |acc, vault| {
                acc.checked_add(vault.borrowed_amount).unwrap()
            })
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

    pub fn record_update_vault(
        &mut self,
        vault_id: VaultId,
        new_owner: Option<Account>,
        fee_bucket: Option<FeeBucket>,
    ) {
        if let Some(new_owner) = new_owner {
            match self.vault_id_to_vault.get_mut(&vault_id) {
                Some(vault) => {
                    vault.owner = new_owner;
                }
                None => panic!("attempted to update unkown vault"),
            };
        }
        if let Some(fee_bucket) = fee_bucket {
            match self.vault_id_to_vault.get_mut(&vault_id) {
                Some(vault) => {
                    vault.fee_bucket = fee_bucket;
                }
                None => panic!("attempted to update unkown vault"),
            };
        }
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
            None => panic!("attempted to add margin to unkown vault"),
        };
    }

    pub fn record_repay_debt_to_vault(&mut self, vault_id: VaultId, debt_repayed: USDG) {
        match self.vault_id_to_vault.get_mut(&vault_id) {
            Some(vault) => {
                vault.borrowed_amount = vault.borrowed_amount.checked_sub(debt_repayed).unwrap();
            }
            None => panic!("attempted to repay to unkown vault"),
        };
    }

    pub fn record_close_vault(&mut self, vault_id: VaultId) {
        let vault = self.remove_vault(vault_id);
        let transfer_id = self.increment_transfer_id();
        assert!(self
            .pending_transfers
            .insert(
                transfer_id,
                PendingTransfer {
                    transfer_id,
                    amount: vault.margin_amount.0,
                    receiver: vault.owner,
                    unit: Unit::GLDT,
                }
            )
            .is_none());
    }

    pub fn record_process_pending_transfer(&mut self, transfer_id: u64) {
        assert!(self.pending_transfers.remove(&transfer_id).is_some());
    }

    pub fn record_claimed_returns(&mut self, from: Account, amount: GLDT) {
        match self.liquidation_return.entry(from) {
            Occupied(mut entry) => {
                *entry.get_mut() = entry.get().checked_sub(amount).unwrap();
                if *entry.get() == GLDT::ZERO {
                    entry.remove_entry();
                }
            }
            Vacant(_) => ic_cdk::trap("cannot claim from unknow principal"),
        }
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

    pub fn remove_vault(&mut self, vault_id: u64) -> Vault {
        let vault = self
            .vault_id_to_vault
            .remove(&vault_id)
            .expect("BUG: tried to remove unknown vault");
        assert!(self
            .account_to_vault_ids
            .get_mut(&vault.owner)
            .expect("BUG: key should exist")
            .remove(&vault_id));
        assert!(self
            .fee_bucket_to_vault_ids
            .get_mut(&vault.fee_bucket)
            .expect("BUG: key should exist")
            .remove(&vault_id));
        vault
    }

    pub fn total_usdg_in_liquidation_pool(&self) -> USDG {
        self.liquidation_pool
            .values()
            .fold(USDG::ZERO, |acc, &amount| acc.checked_add(amount).unwrap())
    }

    pub fn total_gldt_in_returns(&self) -> GLDT {
        self.liquidation_return
            .values()
            .fold(GLDT::ZERO, |acc, &amount| acc.checked_add(amount).unwrap())
    }

    fn lp_remove_debt_add_returns(
        &mut self,
        owner: Account,
        usdg_to_debit: USDG,
        gldt_reward: GLDT,
    ) {
        match self.liquidation_pool.entry(owner) {
            Occupied(mut lp_entry) => {
                *lp_entry.get_mut() = lp_entry.get().checked_sub(usdg_to_debit).unwrap();
                if *lp_entry.get() == USDG::ZERO {
                    lp_entry.remove();
                }
            }
            Vacant(_) => {
                ic_cdk::trap("bug: principal not found in liquidation_pool");
            }
        }
        self.liquidation_return
            .entry(owner)
            .and_modify(|v| *v = v.checked_add(gldt_reward).unwrap())
            .or_insert(gldt_reward);
    }

    pub fn get_biggest_liquidity_provider(&self) -> Option<Account> {
        self.liquidation_pool
            .iter()
            .max_by_key(|(_, usdg_balance)| *usdg_balance)
            .map(|(account, _)| *account)
    }

    pub fn get_most_margin_vault(&self) -> Option<VaultId> {
        self.vault_id_to_vault
            .iter()
            .max_by_key(|(_, vault)| vault.margin_amount)
            .map(|(vault_id, _)| *vault_id)
    }

    pub fn get_most_debt_vault(&self) -> Option<VaultId> {
        self.vault_id_to_vault
            .iter()
            .max_by_key(|(_, vault)| vault.borrowed_amount)
            .map(|(vault_id, _)| *vault_id)
    }

    pub fn record_liquidate_vault_liquidation_pool(&mut self, vault_id: VaultId) {
        let vault = self.get_vault(vault_id).expect("vault should exist");
        let total_provided_amount: USDG = self.total_usdg_in_liquidation_pool();
        assert!(total_provided_amount >= vault.borrowed_amount);

        let mut debt_allocated = USDG::ZERO;
        let mut reward_allocated = GLDT::ZERO;

        let liquidation_pool = self.liquidation_pool.clone();
        for (owner, provided_amount) in liquidation_pool.iter() {
            let share: Factor = provided_amount.checked_div(total_provided_amount).unwrap();
            let gldt_reward = vault.margin_amount.checked_mul(share).unwrap();
            reward_allocated = reward_allocated.checked_add(gldt_reward).unwrap();
            let usdg_to_debit = vault.borrowed_amount.checked_mul(share).unwrap();
            debt_allocated = debt_allocated.checked_add(usdg_to_debit).unwrap();
            self.lp_remove_debt_add_returns(*owner, usdg_to_debit, gldt_reward);
        }

        if vault.borrowed_amount > debt_allocated {
            let usdg_remainder = vault.borrowed_amount.checked_sub(debt_allocated).unwrap();
            match self
                .liquidation_pool
                .entry(self.get_biggest_liquidity_provider().unwrap())
            {
                Occupied(mut lp_entry) => {
                    *lp_entry.get_mut() = lp_entry.get().checked_sub(usdg_remainder).unwrap();
                    if *lp_entry.get() == USDG::ZERO {
                        lp_entry.remove();
                    }
                }
                Vacant(_) => {
                    ic_cdk::trap("bug: principal not found in liquidation_pool");
                }
            }
        } else if vault.borrowed_amount < debt_allocated {
            let extra_usdg = debt_allocated.checked_sub(vault.borrowed_amount).unwrap();
            match self
                .liquidation_pool
                .entry(self.get_biggest_liquidity_provider().unwrap())
            {
                Occupied(mut lp_entry) => {
                    *lp_entry.get_mut() = lp_entry.get().checked_add(extra_usdg).unwrap();
                }
                Vacant(_) => {
                    ic_cdk::trap("bug: principal not found in liquidation_pool");
                }
            }
        }

        if vault.margin_amount > reward_allocated {
            let gldt_remainder = vault.margin_amount.checked_sub(reward_allocated).unwrap();
            self.liquidation_return
                .entry(self.get_biggest_liquidity_provider().unwrap())
                .and_modify(|v| *v = v.checked_add(gldt_remainder).unwrap())
                .or_insert(gldt_remainder);
        } else if reward_allocated > vault.margin_amount {
            let extra_gldt = reward_allocated.checked_sub(vault.margin_amount).unwrap();
            self.liquidation_return
                .entry(self.get_biggest_liquidity_provider().unwrap())
                .and_modify(|v| *v = v.checked_sub(extra_gldt).unwrap());
        }

        self.remove_vault(vault_id);
    }

    pub fn record_redistribute_vault(&mut self, target_vault_id: VaultId) {
        let target_vault = self
            .vault_id_to_vault
            .get(&target_vault_id)
            .expect("bug: vault not found")
            .clone();

        let vaults = self.vault_id_to_vault.clone();
        assert!(!vaults.is_empty());

        let total_gldt_margin: GLDT = vaults.iter().fold(GLDT::ZERO, |acc, (&vault_id, vault)| {
            if vault_id != target_vault_id {
                acc.checked_add(vault.margin_amount).unwrap()
            } else {
                acc
            }
        });
        assert_ne!(total_gldt_margin, GLDT::ZERO);

        let mut margin_distributed = GLDT::ZERO;
        let mut debt_distributed = USDG::ZERO;

        for (vault_id, vault) in vaults {
            if vault_id != target_vault_id {
                let share: Factor = vault
                    .margin_amount
                    .checked_div(total_gldt_margin)
                    .expect("bug: failed to divide margin amount by total margins");
                let gldt_share_amount = target_vault
                    .margin_amount
                    .checked_mul(share)
                    .expect("bug: failed to get gldt share amount");
                margin_distributed = margin_distributed.checked_add(gldt_share_amount).unwrap();
                let usdg_share_amount = target_vault
                    .borrowed_amount
                    .checked_mul(share)
                    .expect("bug: failed to compute usdg share amount");
                debt_distributed = debt_distributed.checked_add(usdg_share_amount).unwrap();
                match self.vault_id_to_vault.entry(vault_id) {
                    Occupied(mut vault_entry) => {
                        vault_entry.get_mut().margin_amount = vault_entry
                            .get()
                            .margin_amount
                            .checked_add(gldt_share_amount)
                            .unwrap();
                        vault_entry.get_mut().borrowed_amount = vault_entry
                            .get()
                            .borrowed_amount
                            .checked_add(usdg_share_amount)
                            .unwrap();
                    }
                    Vacant(_) => panic!("bug: vault not found"),
                }
            }
        }

        if target_vault.borrowed_amount > debt_distributed {
            let debt_remainder = target_vault
                .borrowed_amount
                .checked_sub(debt_distributed)
                .unwrap();
            match self
                .vault_id_to_vault
                .entry(self.get_most_debt_vault().unwrap())
            {
                Occupied(mut vault_entry) => {
                    vault_entry.get_mut().borrowed_amount = vault_entry
                        .get()
                        .borrowed_amount
                        .checked_add(debt_remainder)
                        .unwrap();
                }
                Vacant(_) => panic!("bug: vault not found"),
            }
        } else if debt_distributed > target_vault.borrowed_amount {
            let extra_debt = debt_distributed
                .checked_sub(target_vault.borrowed_amount)
                .unwrap();
            match self
                .vault_id_to_vault
                .entry(self.get_most_debt_vault().unwrap())
            {
                Occupied(mut vault_entry) => {
                    vault_entry.get_mut().borrowed_amount = vault_entry
                        .get()
                        .borrowed_amount
                        .checked_sub(extra_debt)
                        .unwrap();
                }
                Vacant(_) => panic!("bug: vault not found"),
            }
        }

        if target_vault.margin_amount > margin_distributed {
            let margin_remainder = target_vault
                .margin_amount
                .checked_sub(margin_distributed)
                .unwrap();
            match self
                .vault_id_to_vault
                .entry(self.get_most_margin_vault().unwrap())
            {
                Occupied(mut vault_entry) => {
                    vault_entry.get_mut().margin_amount = vault_entry
                        .get()
                        .margin_amount
                        .checked_add(margin_remainder)
                        .unwrap();
                }
                Vacant(_) => panic!("bug: vault not found"),
            }
        } else if margin_distributed > target_vault.margin_amount {
            let extra_margin = margin_distributed
                .checked_sub(target_vault.margin_amount)
                .unwrap();
            match self
                .vault_id_to_vault
                .entry(self.get_most_margin_vault().unwrap())
            {
                Occupied(mut vault_entry) => {
                    vault_entry.get_mut().margin_amount = vault_entry
                        .get()
                        .margin_amount
                        .checked_sub(extra_margin)
                        .unwrap();
                }
                Vacant(_) => panic!("bug: vault not found"),
            }
        }

        self.remove_vault(target_vault_id);
    }

    pub fn get_ordered_vault_ids(&self, gold_price: GoldPrice) -> Vec<VaultId> {
        let mut sorted_vaults = Vec::new();

        for vaults in self.fee_bucket_to_vault_ids.values() {
            let mut vaults: Vec<&Vault> = vaults
                .iter()
                .map(|id| self.vault_id_to_vault.get(id).unwrap())
                .collect();

            vaults.sort_by(|a, b| {
                a.compute_collateral_ratio(gold_price)
                    .partial_cmp(&b.compute_collateral_ratio(gold_price))
                    .unwrap_or(Ordering::Equal)
            });

            let vault_ids: Vec<VaultId> = vaults.iter().map(|vault| vault.vault_id).collect();
            sorted_vaults.extend(vault_ids);
        }

        sorted_vaults
    }

    fn deduct_amount_from_vault(&mut self, vault_id: VaultId, margin: GLDT, debt: USDG) {
        match self.vault_id_to_vault.get_mut(&vault_id) {
            Some(vault) => {
                vault.borrowed_amount = vault.borrowed_amount.checked_sub(debt).unwrap();
                vault.margin_amount = vault.margin_amount.checked_sub(margin).unwrap();
            }
            None => ic_cdk::trap("cannot deduct from unknown vault"),
        }
    }

    pub fn record_redemption(
        &mut self,
        from: Account,
        amount: USDG,
        gold_price: GoldPrice,
    ) -> GLDT {
        let fee_amount = amount
            .checked_mul(get_redemption_fee(amount, self.total_usdg_debt()))
            .unwrap();
        self.reserve_usdg = self.reserve_usdg.checked_add(fee_amount).unwrap();
        let mut amount_to_convert = amount.checked_sub(fee_amount).unwrap();

        let vault_ids = self.get_ordered_vault_ids(gold_price);

        let mut index: usize = 0;
        let mut total_redeemed_gldt = GLDT::ZERO;
        while amount_to_convert > USDG::ZERO && index < vault_ids.len() {
            let vault = self.vault_id_to_vault.get(&vault_ids[index]).unwrap();
            let redeemable_amount = vault.borrowed_amount.min(amount_to_convert);
            amount_to_convert = amount_to_convert.checked_sub(redeemable_amount).unwrap();
            let redeemable_gldt_amount: GLDT =
                redeemable_amount.checked_div_rate(gold_price).unwrap();
            total_redeemed_gldt = total_redeemed_gldt
                .checked_add(redeemable_gldt_amount)
                .unwrap();
            self.deduct_amount_from_vault(
                vault_ids[index],
                redeemable_gldt_amount,
                redeemable_amount,
            );
            index += 1;
        }
        assert_eq!(amount_to_convert, USDG::ZERO);

        let transfer_id = self.increment_transfer_id();
        assert!(self
            .pending_transfers
            .insert(
                transfer_id,
                PendingTransfer {
                    transfer_id,
                    amount: total_redeemed_gldt.0,
                    receiver: from,
                    unit: Unit::GLDT,
                }
            )
            .is_none());

        total_redeemed_gldt
    }

    /// Checks whether the internal state of the core canister matches the other state
    /// semantically (the state holds the same data, but maybe in a slightly
    /// different form).
    pub fn check_semantically_eq(&self, other: &Self) -> Result<(), String> {
        use crate::memory::total_event_count;
        ensure_eq!(
            total_event_count(),
            total_event_count(),
            "total_event_count does not match"
        );
        ensure_eq!(
            self.next_vault_id,
            other.next_vault_id,
            "next_vault_id does not match"
        );
        ensure_eq!(
            self.next_transfer_id,
            other.next_transfer_id,
            "next_transfer_id does not match"
        );
        ensure_eq!(
            self.vault_id_to_vault,
            other.vault_id_to_vault,
            "vault_id_to_vault does not match"
        );
        ensure_eq!(
            self.fee_bucket_to_vault_ids,
            other.fee_bucket_to_vault_ids,
            "fee_bucket_to_vault_ids does not match"
        );
        ensure_eq!(
            self.account_to_vault_ids,
            other.account_to_vault_ids,
            "account_to_vault_ids does not match"
        );
        ensure_eq!(
            self.liquidation_pool,
            other.liquidation_pool,
            "liquidation_pool does not match"
        );
        ensure_eq!(
            self.liquidation_return,
            other.liquidation_return,
            "liquidation_return does not match"
        );
        ensure_eq!(
            self.pending_transfers,
            other.pending_transfers,
            "pending_transfers does not match"
        );
        ensure_eq!(
            self.reserve_usdg,
            other.reserve_usdg,
            "reserve_usdg does not match"
        );

        Ok(())
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
