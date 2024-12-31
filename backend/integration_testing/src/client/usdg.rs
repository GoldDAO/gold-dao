use crate::{generate_pocket_query_call, generate_pocket_update_call};
use usdg_minter_api::queries::{get_lp_position, get_vaults_by_account};
use usdg_minter_api::updates::{
    add_margin_to_vault, borrow_from_vault, close_vault, deposit_liquidity, open_vault,
    repay_debt_to_vault, withdraw_liquidity, claim_returns
};

generate_pocket_update_call!(open_vault);
generate_pocket_update_call!(close_vault);
generate_pocket_update_call!(claim_returns);
generate_pocket_update_call!(repay_debt_to_vault);
generate_pocket_update_call!(borrow_from_vault);
generate_pocket_update_call!(add_margin_to_vault);
generate_pocket_update_call!(deposit_liquidity);
generate_pocket_update_call!(withdraw_liquidity);
generate_pocket_query_call!(get_vaults_by_account);
generate_pocket_query_call!(get_lp_position);
