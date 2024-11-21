use crate::numeric::{GoldPrice, GLDT, USDG};
use crate::transfer::{PendingTransfer, TransferId, Unit};
use crate::vault::{FeeBucket, Vault, VaultId};
use crate::{DEFAULT_GOLD_PRICE, MINIMUM_COLLATERAL_RATIO};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
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

    // Pending transfers to be processed
    pub pending_transfers: BTreeMap<TransferId, PendingTransfer>,

    // 0.01g of gold price in USD
    pub one_centigram_of_gold_price: GoldPrice,

    // Canister ids
    pub usdg_ledger_id: Principal,
    pub gldt_ledger_id: Principal,
    pub gold_dao_governance_id: Principal,
    pub xrc_id: Principal,
}

impl State {
    pub fn new(init_arg: InitArgument) -> State {
        State {
            next_vault_id: 0,
            next_transfer_id: 0,
            fee_bucket_to_vault_ids: Default::default(),
            account_to_vault_ids: Default::default(),
            vault_id_to_vault: Default::default(),
            pending_transfers: Default::default(),
            one_centigram_of_gold_price: DEFAULT_GOLD_PRICE,
            usdg_ledger_id: init_arg.usdg_ledger_id,
            gldt_ledger_id: init_arg.gldt_ledger_id,
            gold_dao_governance_id: init_arg.gold_dao_governance_id,
            xrc_id: init_arg.xrc_id,
        }
    }

    pub fn check_open_vault_args_validity(
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
            .into_iter()
            .map(|vault| self.vault_id_to_vault.get(&vault).unwrap())
            .cloned()
            .collect()
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

    pub fn active_vault_count(&self) -> usize {
        self.vault_id_to_vault.len()
    }

    pub fn record_vault_creation(
        &mut self,
        owner: Account,
        borrowed_amount: USDG,
        margin_amount: GLDT,
        fee_bucket: impl Into<FeeBucket>,
    ) -> VaultId {
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
