pub mod init;
pub mod post_upgrade;

use crate::init::InitArgs;
use crate::post_upgrade::UpgradeArgs;

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Args {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}
