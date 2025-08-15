use self::types::state::RuntimeStateV0;
use crate::{
    model::proposal_system::ProposalSystem,
    state::{Data, RuntimeState},
    utils::TimeInterval,
};
use candid::Principal;
use std::collections::BTreeMap;
use time::Time;
pub mod types;

impl From<RuntimeStateV0> for RuntimeState {
    fn from(mut old_state: RuntimeStateV0) -> Self {
        Self {
            env: old_state.env,
            data: Data {
                gldt_ledger_id: old_state.data.gldt_ledger_id,
                goldao_ledger_id: old_state.data.goldao_ledger_id,
                authorized_principals: old_state.data.authorized_principals,
                whitelist: vec![
                    Principal::from_text(
                        "wslab-6xddk-alf2g-dpo7r-pkekk-cehij-xsl4s-gbthe-k2bvu-gvmn4-6qe",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "2s6yw-v2gw3-ftsve-wojsm-ifnjs-j25mm-wzvru-zpac4-hkd36-3nkjf-3qe",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "xrjbg-ib65u-u2mfw-jn4g7-gquzt-tncfe-cbw44-steji-nww53-p5552-3ae",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "ylske-4jich-5iiqe-tb2q2-aobmm-nt3sr-oplxx-6gojb-4k3fl-dbwku-3ae",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "wzvtt-336ts-bwvcb-xw2jf-bxpuf-e5afd-zm2im-zb7ir-iacsz-65wat-pqe",
                    )
                    .unwrap(),
                    Principal::from_text(
                        "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae",
                    )
                    .unwrap(),
                ],
                stake_system: old_state.data.stake_system,
                goldao_sns_rewards_canister_id: old_state.data.goldao_sns_rewards_canister_id,
                goldao_sns_governance_canister_id: old_state.data.goldao_sns_governance_canister_id,
                neuron_system: old_state.data.neuron_system,
                unallocated_rewards_pool: old_state.data.unallocated_rewards_pool,
                allocate_rewards_interval: Some(TimeInterval {
                    weekday: Some("Thursday".to_string()),
                    start_hour: 11,
                    end_hour: 12,
                }),
                processing_rewards_pool: old_state.data.processing_rewards_pool,
                allocated_rewards_pool: old_state.data.allocated_rewards_pool,
                reward_claim_interval: Some(TimeInterval {
                    weekday: Some("Thursday".to_string()),
                    start_hour: 10,
                    end_hour: 11,
                }),
                principal_guards: old_state.data.principal_guards,
                proposal_system: old_state.data.proposal_system,
            },
        }
    }
}
