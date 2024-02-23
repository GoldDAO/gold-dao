use candid::CandidType;
use ic_ledger_types::Subaccount;
use ledger_utils::principal_to_legacy_account_id;
use nns_governance_canister::types::{ neuron::DissolveState, Neuron };
use serde::Serialize;
use utils::consts::NNS_GOVERNANCE_CANISTER_ID;

#[derive(CandidType, Serialize, Debug, PartialEq, Eq)]
pub struct NeuronWithMetric {
    pub id: u64,
    pub deposit_account: String,
    pub staked_amount: u64,
    pub maturity: u64,
    pub dissolve_delay: u64,
    pub dissolving: bool,
}

impl From<Neuron> for NeuronWithMetric {
    fn from(neuron: Neuron) -> Self {
        let mut dissolve_delay = 0u64;
        let mut dissolving = false;

        if let Some(dissolve_info) = neuron.dissolve_state {
            match dissolve_info {
                DissolveState::WhenDissolvedTimestampSeconds(ts) => {
                    dissolving = true;
                    dissolve_delay = ts;
                }
                DissolveState::DissolveDelaySeconds(ts) => {
                    dissolve_delay = ts;
                }
            }
        }

        let subaccount_bytes: Result<[u8; 32], _> = neuron.account.try_into();
        let deposit_account = match subaccount_bytes {
            Ok(bytes) =>
                principal_to_legacy_account_id(
                    NNS_GOVERNANCE_CANISTER_ID,
                    Some(Subaccount(bytes))
                ).to_hex(),
            Err(_) => "unknown".to_string(),
        };
        Self {
            id: neuron.id.map_or(0, |id| id.id),
            deposit_account,
            staked_amount: neuron.cached_neuron_stake_e8s,
            maturity: neuron.maturity_e8s_equivalent,
            dissolve_delay,
            dissolving,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nns_governance_canister::types::{ Neuron, NeuronId };

    use super::NeuronWithMetric;

    #[test]
    fn convert_neuron_to_neuron_metric() {
        let neuron = Neuron {
            id: Some(NeuronId { id: 17_481_076_647_658_761_488 }),
            account: vec![
                149,
                128,
                178,
                23,
                182,
                54,
                48,
                115,
                178,
                174,
                154,
                119,
                21,
                182,
                104,
                106,
                141,
                106,
                190,
                141,
                3,
                144,
                216,
                56,
                228,
                185,
                230,
                194,
                1,
                119,
                126,
                193
            ],
            controller: None,
            hot_keys: vec![],
            cached_neuron_stake_e8s: 0,
            neuron_fees_e8s: 0,
            created_timestamp_seconds: 0,
            aging_since_timestamp_seconds: 0,
            spawn_at_timestamp_seconds: None,
            followees: HashMap::default(),
            recent_ballots: vec![],
            kyc_verified: false,
            maturity_e8s_equivalent: 0,
            staked_maturity_e8s_equivalent: None,
            auto_stake_maturity: None,
            not_for_profit: false,
            joined_community_fund_timestamp_seconds: None,
            known_neuron_data: None,
            dissolve_state: None,
        };

        let result = NeuronWithMetric::from(neuron);

        let expeted_result = NeuronWithMetric {
            id: 17_481_076_647_658_761_488,
            deposit_account: "6601afb37d5807c9ed17c8343bb1c7180f98eca73a64727f56134c720cf0304a".to_string(),
            staked_amount: 0,
            maturity: 0,
            dissolve_delay: 0,
            dissolving: false,
        };

        assert_eq!(result, expeted_result)
    }
}
