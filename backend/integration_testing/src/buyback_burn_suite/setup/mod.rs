use self::setup::{ BuybackBurnTestEnv, BuybackBurnTestEnvBuilder };
pub mod setup;
pub mod setup_buyback_burn;
pub mod setup_ledger;
use candid::Nat;

pub fn default_test_setup() -> BuybackBurnTestEnv {
    BuybackBurnTestEnvBuilder::new()
        .add_token_ledger("ICP", &mut vec![], Nat::from(10_000u64))
        .build()
}
