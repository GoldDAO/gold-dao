use self::setup::{CyclesManagerEnv, CyclesManagerTestEnvBuilder};

pub mod setup;
pub mod setup_burner;
pub mod setup_cycles_manager;
pub mod setup_cycles_minting;
pub mod setup_icp_ledger;
pub mod setup_sns_root;

pub fn default_top_up_test_setup() -> CyclesManagerEnv {
    CyclesManagerTestEnvBuilder::new()
        // .with_icp_burn_amount(0)
        .build()
}

pub fn default_burn_icp_into_cycles_test_setup() -> CyclesManagerEnv {
    CyclesManagerTestEnvBuilder::new()
        .with_min_cycles_balance(200_000_000_000_000)
        .build()
}

pub fn default_full_flow() -> CyclesManagerEnv {
    CyclesManagerTestEnvBuilder::new()
        .with_min_cycles_balance(2_000_000_000_000_000)
        .with_icp_burn_amount(15_000_000_000)
        .with_icp_burn_amount(2_000_000_000_000_000)
        .build()
}
