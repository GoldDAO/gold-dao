use crate::{generate_pocket_query_call, generate_pocket_update_call};

use usdg_minter_api::queries::get_vaults_by_account;
use usdg_minter_api::updates::open_vault;

generate_pocket_update_call!(open_vault);
generate_pocket_query_call!(get_vaults_by_account);
