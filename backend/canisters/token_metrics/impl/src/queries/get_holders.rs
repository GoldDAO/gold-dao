use crate::state::read_state;
use ic_cdk_macros::query;
pub use token_metrics_api::queries::get_holders::{
    Args as GetHoldersArgs, Response as GetHoldersResponse,
};

#[query]
fn get_holders(args: GetHoldersArgs) -> GetHoldersResponse {
    let response = read_state(|state| {
        let list = if args.merge_accounts_to_principals {
            &state.data.merged_wallets_list
        } else {
            &state.data.wallets_list
        };

        let mut current_offset = args.offset;
        let mut result = Vec::new();

        for entry in list.iter() {
            if current_offset > 0 {
                current_offset -= 1;
                continue;
            }

            if result.len() >= (args.limit as usize) {
                break;
            }

            result.push((entry.0, entry.1));
        }
        result
    });

    GetHoldersResponse {
        data: response.clone(),
        current_offset: args.offset,
        limit: args.limit,
        total_count: response.len() as usize,
    }
}
