use crate::buyback_burn_suite::setup::setup_buyback_burn::setup_buyback_burn_canister;
use crate::utils::random_principal;
use ic_ledger_types::Tokens;
use candid::Principal;
use pocket_ic::{ PocketIc, PocketIcBuilder };
use buyback_burn_api::Args;
use types::{ BuildVersion, CanisterId, TokenInfo };
use buyback_burn_api::init::TokenAndPool;
use icrc_ledger_types::icrc1::account::Account;
use std::collections::HashMap;
use candid::Nat;
use crate::buyback_burn_suite::setup::setup_ledger::setup_ledgers;

pub struct BuybackBurnTestEnv {
    pub controller: Principal,
    pub buyback_burn_id: CanisterId,
    pub pic: PocketIc,
}

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
impl Debug for BuybackBurnTestEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuybackBurnTestEnv")
            .field("controller", &self.controller.to_text())
            .field("buyback_burn_id", &self.buyback_burn_id.to_text())
            .finish()
    }
}

pub struct BuybackBurnTestEnvBuilder {
    controller: Principal,
    buyback_burn_id: CanisterId,
    // Ledger parameters
    token_symbols: Vec<String>,
    initial_ledger_accounts: Vec<(Account, Nat)>,
    ledger_fees: HashMap<String, Nat>,
}

impl Default for BuybackBurnTestEnvBuilder {
    fn default() -> Self {
        Self {
            controller: random_principal(),
            buyback_burn_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 4]),
            token_symbols: vec![],
            initial_ledger_accounts: vec![],
            ledger_fees: HashMap::new(),
        }
    }
}

impl BuybackBurnTestEnvBuilder {
    pub fn new() -> Self {
        BuybackBurnTestEnvBuilder::default()
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn add_token_ledger(
        mut self,
        symbol: &str,
        initial_balances: &mut Vec<(Account, Nat)>,
        transaction_fee: Nat
    ) -> Self {
        self.token_symbols.push(symbol.to_string());
        self.initial_ledger_accounts.append(initial_balances);
        self.ledger_fees.insert(symbol.to_string(), transaction_fee);
        self
    }

    pub fn build(&mut self) -> BuybackBurnTestEnv {
        let mut pic = PocketIcBuilder::new().with_sns_subnet().with_application_subnet().build();

        let sns_subnet = pic.topology().get_sns().unwrap();
        self.buyback_burn_id = pic.create_canister_on_subnet(
            Some(self.controller.clone()),
            None,
            sns_subnet
        );

        let token_ledgers = setup_ledgers(
            &pic,
            self.controller.clone(),
            self.token_symbols.clone(),
            self.initial_ledger_accounts.clone(),
            self.ledger_fees.clone()
        );

        let icp_ledger_canister_id = token_ledgers.get("icp_ledger_canister_id").unwrap().clone();

        let buyback_burn_init_args = Args::Init(buyback_burn_api::init::InitArgs {
            test_mode: true,
            version: BuildVersion::min(),
            commit_hash: "Hash".to_string(),
            authorized_principals: vec![self.controller.clone()],
            gldgov_token_info: TokenInfo {
                ledger_id: Principal::anonymous(),
                fee: 10000,
                decimals: 8,
            },
            tokens: vec![TokenAndPool {
                token: TokenInfo {
                    ledger_id: icp_ledger_canister_id,
                    fee: 10000,
                    decimals: 8,
                },
                swap_pool_id: Principal::anonymous(),
            }],
            buyback_burn_interval_in_secs: 14400,
            icp_swap_canister_id: Principal::anonymous(),
            burn_rate: 33,
            min_burn_amount: Tokens::from_e8s(30000_u64),
        });

        let buyback_burn_canister_id = setup_buyback_burn_canister(
            &mut pic,
            self.buyback_burn_id,
            buyback_burn_init_args,
            self.controller
        );

        BuybackBurnTestEnv {
            controller: self.controller,
            buyback_burn_id: buyback_burn_canister_id,
            pic,
        }
    }
}
