use crate::lifecycle::init_canister;
use crate::memory::get_upgrades_memory;
use crate::migrations::types::state::RuntimeStateV0;
use crate::state::RuntimeState;
use bity_ic_canister_logger::LogEntry;
use bity_ic_canister_tracing_macros::trace;
use bity_ic_stable_memory::get_reader;
use ic_cdk_macros::post_upgrade;
use ic_ledger_types::AccountIdentifier;
use icp_neuron_api_canister::Args;
use tracing::info;

const CYCLE_MANAGEMENT_ACCOUNT: &str =
    "a51ceabd4d86c16c94936db0422d9b814b4f20e58fa013aeace0053af2305e8c";

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    match args {
        Args::Init(_) =>
            panic!(
                "Cannot upgrade the canister with an Init argument. Please provide an Upgrade argument."
            ),
        Args::Upgrade(upgrade_args) => {
            let memory = get_upgrades_memory();
            let reader = get_reader(&memory);

            // uncomment these lines if you want to do a normal upgrade
            let (mut state, logs, traces): (RuntimeState, Vec<LogEntry>, Vec<LogEntry>) = bity_ic_serializer
                ::deserialize(reader)
                .unwrap();

            // uncomment these lines if you want to do an upgrade with migration
            // let (runtime_state_v0, logs, traces): (
            //     RuntimeStateV0,
            //     Vec<LogEntry>,
            //     Vec<LogEntry>,
            // ) = serializer::deserialize(reader).unwrap();
            // let mut state = RuntimeState::from(runtime_state_v0);

            state.env.set_version(upgrade_args.version);
            state.env.set_commit_hash(upgrade_args.commit_hash);

            let cycle_management_account = AccountIdentifier::from_hex(CYCLE_MANAGEMENT_ACCOUNT)
                .expect("CYCLE_MANAGEMENT_ACCOUNT is a valid account identifier");
            state.data.cycle_management_account = vec![cycle_management_account];

            bity_ic_canister_logger::init_with_logs(state.env.is_test_mode(), logs, traces);
            init_canister(state);

            info!(version = %upgrade_args.version, "Post-upgrade complete");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CYCLE_MANAGEMENT_ACCOUNT;
    use ic_ledger_types::AccountIdentifier;

    #[test]
    fn cycle_management_account_is_a_valid_account_identifier() {
        let account = AccountIdentifier::from_hex(CYCLE_MANAGEMENT_ACCOUNT).unwrap();
        assert_eq!(account.to_hex(), CYCLE_MANAGEMENT_ACCOUNT);
    }
}
