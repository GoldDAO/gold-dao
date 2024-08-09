use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_holders::{
    Args as GetHoldersArgs,
    Response as GetHoldersResponse,
};
use crate::state::read_state;

#[query]
fn get_holders(args: GetHoldersArgs) -> GetHoldersResponse {
    let mut result = Vec::new();
    let mut current_offset = args.offset;
    let list = read_state(|state| {
        if args.merge_accounts_to_principals {
            state.data.merged_wallets_list.clone()
        } else {
            state.data.wallets_list.clone()
        }
    });
    for (key, value) in list.iter() {
        if current_offset > 0 {
            current_offset -= 1;
            continue;
        }

        if result.len() >= (args.limit as usize) {
            break;
        }

        result.push((key.clone(), value.clone()));
    }

    GetHoldersResponse {
        data: result,
        current_offset: args.offset,
        limit: args.limit,
        total_count: list.len()
    }
}
