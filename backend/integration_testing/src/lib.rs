use candid::{ CandidType, Deserialize, Principal };
use serde::Serialize;

#[derive(Deserialize, CandidType)]
pub struct Args {
    test_mode: bool,
    pocket_ic: bool,
}

mod sns_init_payload;
#[cfg(test)]
mod tests {
    use std::{ ffi::OsString };

    use ic_cdk::api::management_canister::main::CanisterId;
    use pocket_ic::{ common::rest::SubnetConfigSet, PocketIc, PocketIcBuilder, WasmResult };
    use candid::{ decode_one, encode_args, encode_one, Principal };
    use utils::consts::SNS_GOVERNANCE_CANISTER_ID_STAGING;

    use crate::{
        sns_init_payload::{ FractionalDeveloperVotingPower, SnsInitArg, SnsInitPayload },
        Args,
    };

    // 10T cycles
    const INIT_CYCLES: u128 = 10_000_000_000_000;

    fn update_call(ic: &PocketIc, can_id: CanisterId, method: &str) -> WasmResult {
        ic.update_call(can_id, Principal::anonymous(), method, encode_one(()).unwrap()).expect(
            "Failed to call counter canister"
        )
    }

    fn query_call(ic: &PocketIc, can_id: CanisterId, method: &str) -> WasmResult {
        ic.query_call(can_id, Principal::anonymous(), method, encode_one(()).unwrap()).expect(
            "Failed to query canister"
        )
    }

    fn get_rewards_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString =
            "../canisters/sns_rewards/target/wasm32-unknown-unknown/release/sns_rewards.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    fn get_governance_canister_wasm() -> Vec<u8> {
        let wasm_path: OsString = "./sns-governance-canister.wasm".into();
        std::fs::read(wasm_path).unwrap()
    }

    #[test]
    #[should_panic(expected = "is out of cycles")]
    fn test_sanity() {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        let wasm = b"\x00\x61\x73\x6d\x01\x00\x00\x00".to_vec();
        pic.install_canister(canister_id, wasm.clone(), vec![], None);
    }

    // test neuron sync works - happy path
    #[test]
    fn calls_a_dummy_method() {
        let pic = PocketIc::new();

        // Create a canister and charge it with 2T cycles.
        let can_id = pic.create_canister();
        pic.add_cycles(can_id, INIT_CYCLES);

        // Install the counter canister wasm file on the canister.
        let wasm = get_rewards_canister_wasm();
        let init_args = Args { test_mode: true, pocket_ic: true };
        pic.install_canister(can_id, wasm, encode_one(init_args).unwrap(), None);

        // Make some calls to the canister.
        let reply: Result<bool, String> = match update_call(&pic, can_id, "read") {
            WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
            WasmResult::Reject(_) => {
                return;
            }
        };
        assert_eq!(reply.unwrap(), true);
    }

    #[test]
    fn synchronise_neurons_happy_path() {
        let pic = PocketIcBuilder::new().with_sns_subnet().build();

        let sns_subnet = pic.topology().get_sns().unwrap();

        let sns_gov_id = pic.create_canister_on_subnet(None, None, sns_subnet);
        pic.add_cycles(sns_gov_id, INIT_CYCLES);

        let sns_controller = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1]).to_string();

        // let sns_gov_init_args = SnsInitPayload {
        //     confirmation_text: Some("Welcome to the jungle baby".to_string()),
        //     transaction_fee_e8s: Some(10000u64),
        //     token_name: Some("Simulation Governance".to_string()),
        //     token_symbol: Some("SIMG".to_string()),
        //     proposal_reject_cost_e8s: Some(10000u64),
        //     neuron_minimum_stake_e8s: Some(10000u64),
        //     fallback_controller_principal_ids: vec![sns_controller.clone()],
        //     logo: Some("data:image/png;base64,iVBORw0".to_string()),
        //     url: Some("https://google.com".to_string()),
        //     name: Some("Simulation Gov".to_string()),
        //     description: Some("Simulation gov desc".to_string()),
        //     neuron_minimum_dissolve_delay_to_vote_seconds: Some(1),
        //     initial_reward_rate_basis_points: Some(10u64),
        //     final_reward_rate_basis_points: Some(20u64),
        //     reward_rate_transition_duration_seconds: Some(1u64),
        //     max_dissolve_delay_seconds: Some(1u64),
        //     max_neuron_age_seconds_for_age_bonus: Some(1u64),
        //     max_dissolve_delay_bonus_percentage: Some(10u64),
        //     max_age_bonus_percentage: Some(10u64),
        //     initial_voting_period_seconds: Some(1u64),
        //     wait_for_quiet_deadline_increase_seconds: Some(1u64),
        //     restricted_countries: None,
        //     dapp_canisters: None,
        //     min_participants: Some(1),
        //     min_icp_e8s: Some(1u64),
        //     max_icp_e8s: Some(10_000_000_000u64),
        //     min_direct_participation_icp_e8s: Some(10000u64),
        //     min_participant_icp_e8s: Some(10000u64),
        //     max_direct_participation_icp_e8s: Some(100_000u64),
        //     max_participant_icp_e8s: Some(10000u64),
        //     swap_start_timestamp_seconds: None,
        //     swap_due_timestamp_seconds: Some(32512438014000u64), // year 3000 - hopefully we'll all be gone by then,
        //     neuron_basket_construction_parameters: None,
        //     nns_proposal_id: Some(1),
        //     neurons_fund_participation: None,
        //     neurons_fund_participants: None,
        //     token_logo: Some("data:image/png;base64,iVBORw0".to_string()),
        //     neurons_fund_participation_constraints: None,
        //     initial_token_distribution: Some(
        //         crate::sns_init_payload::InitialTokenDistribution::FractionalDeveloperVotingPower(
        //             FractionalDeveloperVotingPower {
        //                 airdrop_distribution: None,
        //                 developer_distribution: None,
        //                 treasury_distribution: None,
        //                 swap_distribution: None,
        //             }
        //         )
        //     ),
        // };
        let sns_gov_init_args = SnsInitPayload {
            confirmation_text: Some("Welcome to the jungle baby".to_string()),
            transaction_fee_e8s: Some(10000u64),
            token_name: Some("Simulation Governance".to_string()),
            token_symbol: Some("SIMG".to_string()),
            proposal_reject_cost_e8s: Some(10000u64),
            neuron_minimum_stake_e8s: Some(10000u64),
            fallback_controller_principal_ids: vec![sns_controller.clone()],
            logo: Some("data:image/png;base64,iVBORw0".to_string()),
            url: Some("https://google.com".to_string()),
            name: Some("Simulation Gov".to_string()),
            description: Some("Simulation gov desc".to_string()),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(1),
            initial_reward_rate_basis_points: Some(10u64),
            final_reward_rate_basis_points: Some(20u64),
            reward_rate_transition_duration_seconds: Some(1u64),
            max_dissolve_delay_seconds: Some(1u64),
            max_neuron_age_seconds_for_age_bonus: Some(1u64),
            max_dissolve_delay_bonus_percentage: Some(10u64),
            max_age_bonus_percentage: Some(10u64),
            initial_voting_period_seconds: Some(1u64),
            wait_for_quiet_deadline_increase_seconds: Some(1u64),
            restricted_countries: None,
            dapp_canisters: None,
            min_participants: Some(1),
            min_icp_e8s: Some(1u64),
            max_icp_e8s: Some(10_000_000_000u64),
            min_direct_participation_icp_e8s: Some(10000u64),
            min_participant_icp_e8s: Some(10000u64),
            max_direct_participation_icp_e8s: Some(100_000u64),
            max_participant_icp_e8s: Some(10000u64),
            swap_start_timestamp_seconds: None,
            swap_due_timestamp_seconds: Some(32512438014000u64), // year 3000 - hopefully we'll all be gone by then,
            neuron_basket_construction_parameters: None,
            nns_proposal_id: Some(1),
            neurons_fund_participation: None,
            neurons_fund_participants: None,
            token_logo: Some("data:image/png;base64,iVBORw0".to_string()),
            neurons_fund_participation_constraints: None,
            initial_token_distribution: Some(
                crate::sns_init_payload::InitialTokenDistribution::FractionalDeveloperVotingPower(
                    FractionalDeveloperVotingPower {
                        airdrop_distribution: None,
                        developer_distribution: None,
                        treasury_distribution: None,
                        swap_distribution: None,
                    }
                )
            ),
        };
        let init_args_two = SnsInitArg { sns_initialization_parameters: sns_gov_init_args };

        pic.install_canister(
            sns_gov_id,
            get_governance_canister_wasm(),
            encode_one(init_args_two).unwrap(),
            None
        );

        // let pic = PocketIc::from_config(config);
        // // goes on NNS
        // let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        // let actual_canister_id = pic.create_canister_with_id(None, None, canister_id).unwrap();

        // let sns_subnet_id = pic.topology().get_sns().unwrap();

        // // ----------- install fake governance canister ----------
        // let governance_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
        // pic.add_cycles(governance_canister, INIT_CYCLES);

        // let wasm = get_governance_canister_wasm();

        // pic.install_canister(governance_canister, wasm, encode_one(()).unwrap(), None);

        // // install rewards canister
        // let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
        // pic.add_cycles(rewards_canister, INIT_CYCLES);

        // let wasm = get_rewards_canister_wasm();
        // let init_args = Args { test_mode: true, pocket_ic: true };
        // pic.install_canister(rewards_canister, wasm, encode_one(init_args).unwrap(), None);

        // // test rewards canister
        // let reply: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
        //     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
        //     WasmResult::Reject(_) => {
        //         return;
        //     }
        // };
        // // there should be 0 neurons
        // pic.advance_time(std::time::Duration::from_secs(20));
        // assert_eq!(reply, 0);

        // add some neurons

        // advance time for maturity
    }

    // fn synchronise_neurons_happy_path_old() {
    //     // let config = SubnetConfigSet {
    //     //     ..Default::default()
    //     // };
    //     // let pic = PocketIc::from_config(config);
    //     // // goes on NNS
    //     // let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    //     // let actual_canister_id = pic.create_canister_with_id(None, None, canister_id).unwrap();

    //     // let sns_subnet_id = pic.topology().get_sns().unwrap();

    //     // // ----------- install fake governance canister ----------
    //     // let governance_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
    //     // pic.add_cycles(governance_canister, INIT_CYCLES);

    //     // let wasm = get_governance_canister_wasm();

    //     // pic.install_canister(governance_canister, wasm, encode_one(()).unwrap(), None);

    //     // // install rewards canister
    //     // let rewards_canister = pic.create_canister_on_subnet(None, None, sns_subnet_id);
    //     // pic.add_cycles(rewards_canister, INIT_CYCLES);

    //     // let wasm = get_rewards_canister_wasm();
    //     // let init_args = Args { test_mode: true, pocket_ic: true };
    //     // pic.install_canister(rewards_canister, wasm, encode_one(init_args).unwrap(), None);

    //     // // test rewards canister
    //     // let reply: usize = match query_call(&pic, rewards_canister, "get_all_neurons") {
    //     //     WasmResult::Reply(bytes) => decode_one(bytes.as_slice()).unwrap(),
    //     //     WasmResult::Reject(_) => {
    //     //         return;
    //     //     }
    //     // };
    //     // // there should be 0 neurons
    //     // pic.advance_time(std::time::Duration::from_secs(20));
    //     // assert_eq!(reply, 0);

    //     // add some neurons

    //     // advance time for maturity
    // }
}
