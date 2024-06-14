use self::setup::{CyclesManagerEnv, CyclesManagerTestEnvBuilder};

pub mod setup;
pub mod setup_burner;
pub mod setup_cycles_manager;
pub mod setup_cycles_minting;
pub mod setup_icp_ledger;
pub mod setup_sns_root;

pub fn default_test_setup() -> CyclesManagerEnv {
    CyclesManagerTestEnvBuilder::new().build()
}
