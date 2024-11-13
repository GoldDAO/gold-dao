use ic_cdk_macros::init;
pub use sns_rewards_api_canister::Args;
use tracing::info;
use types::{TokenInfo, TokenSymbol};
use utils::env::CanisterEnv;

use crate::state::{Data, RuntimeState};

use super::init_canister;

#[init]
fn init(args: Args) {
    match args {
        Args::Init(init_args) => {
            canister_logger::init(init_args.test_mode);

            let env = CanisterEnv::new(
                init_args.test_mode,
                init_args.version,
                init_args.commit_hash,
            );
            let mut data = Data::default();

            // use staging canister ids
            if init_args.test_mode {
                let icp_ledger_canister_id = init_args.icp_ledger_canister_id;
                let ogy_ledger_canister_id = init_args.ogy_ledger_canister_id;
                let gldgov_ledger_canister_id = init_args.sns_ledger_canister_id;

                if let Ok(token) = TokenSymbol::parse("ICP") {
                    data.tokens.insert(
                        token,
                        TokenInfo {
                            ledger_id: icp_ledger_canister_id,
                            fee: 10_000u64,
                            decimals: 8u64,
                        },
                    );
                }
                if let Ok(token) = TokenSymbol::parse("OGY") {
                    data.tokens.insert(
                        token,
                        TokenInfo {
                            ledger_id: ogy_ledger_canister_id,
                            fee: 200_000u64,
                            decimals: 8u64,
                        },
                    );
                }
                if let Ok(token) = TokenSymbol::parse("GLDGov") {
                    data.tokens.insert(
                        token.clone(),
                        TokenInfo {
                            ledger_id: gldgov_ledger_canister_id,
                            fee: 100_000u64,
                            decimals: 8u64,
                        },
                    );
                }

                data.authorized_principals = vec![init_args.sns_gov_canister_id];
                data.sns_governance_canister = init_args.sns_gov_canister_id;
            }

            let runtime_state = RuntimeState::new(env, data);

            init_canister(runtime_state);

            info!("Init complete.")
        }
        Args::Upgrade(_) => {
            panic!(
                "Cannot initialize the canister with an Upgrade argument. Please provide an Init argument."
            );
        }
    }
}
