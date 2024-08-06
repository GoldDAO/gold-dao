use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use sns_governance_canister::types::{neuron::DissolveState, Neuron};
use types::CanisterId;

#[derive(CandidType, Serialize, Debug, PartialEq, Eq)]
pub struct NeuronWithMetric {
    pub id: Vec<u8>,
    pub deposit_account: DepositAccount,
    pub staked_amount: u64,
    pub maturity: u64,
    pub dissolve_delay: u64,
    pub dissolving: bool,
}

impl NeuronWithMetric {
    pub fn from_neuron_with_sns_gov_id(
        neuron: Neuron,
        sns_governance_canister_id: CanisterId,
    ) -> Self {
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

        let subaccount_bytes: [u8; 32] = neuron.id.clone().unwrap_or_default().into();
        let icrc_account = Account {
            owner: sns_governance_canister_id,
            subaccount: Some(subaccount_bytes),
        };
        let deposit_account = DepositAccount {
            icrc_account,
            icrc_account_as_string: icrc_account.to_string(),
        };

        Self {
            id: neuron.id.map_or(vec![0; 32], |id| id.id),
            deposit_account,
            staked_amount: neuron.cached_neuron_stake_e8s,
            maturity: neuron.maturity_e8s_equivalent,
            dissolve_delay,
            dissolving,
        }
    }
}

#[derive(CandidType, Serialize, Debug, PartialEq, Eq)]
pub struct DepositAccount {
    icrc_account: Account,
    icrc_account_as_string: String,
}

#[cfg(test)]
mod tests {
    use icrc_ledger_types::icrc1::account::Account;
    use sns_governance_canister::types::{Neuron, NeuronId};
    use std::collections::BTreeMap;
    use utils::consts::SNS_GOVERNANCE_CANISTER_ID;

    use crate::types::neuron_metrics::DepositAccount;

    use super::NeuronWithMetric;

    #[test]
    fn convert_neuron_to_neuron_metric() {
        let neuron = Neuron {
            id: Some(NeuronId {
                id: vec![
                    149, 128, 178, 23, 182, 54, 48, 115, 178, 174, 154, 119, 21, 182, 104, 106,
                    141, 106, 190, 141, 3, 144, 216, 56, 228, 185, 230, 194, 1, 119, 126, 193,
                ],
            }),
            permissions: vec![],
            cached_neuron_stake_e8s: 0,
            neuron_fees_e8s: 0,
            created_timestamp_seconds: 0,
            aging_since_timestamp_seconds: 0,
            followees: BTreeMap::default(),
            maturity_e8s_equivalent: 0,
            voting_power_percentage_multiplier: 0,
            source_nns_neuron_id: None,
            staked_maturity_e8s_equivalent: None,
            auto_stake_maturity: None,
            vesting_period_seconds: None,
            disburse_maturity_in_progress: vec![],
            dissolve_state: None,
        };

        let result =
            NeuronWithMetric::from_neuron_with_sns_gov_id(neuron, SNS_GOVERNANCE_CANISTER_ID);

        let expeted_result = NeuronWithMetric {
            id: vec![
                149, 128, 178, 23, 182, 54, 48, 115, 178, 174, 154, 119, 21, 182, 104, 106, 141,
                106, 190, 141, 3, 144, 216, 56, 228, 185, 230, 194, 1, 119, 126, 193,
            ],
            deposit_account: DepositAccount {
                icrc_account: Account {
                    owner: SNS_GOVERNANCE_CANISTER_ID,
                    subaccount: Some([
                        149, 128, 178, 23, 182, 54, 48, 115, 178, 174, 154, 119, 21, 182, 104, 106,
                        141, 106, 190, 141, 3, 144, 216, 56, 228, 185, 230, 194, 1, 119, 126, 193,
                    ]),
                },
                icrc_account_as_string:
                    "tr3th-kiaaa-aaaaq-aab6q-cai-pkvjdri.9580b217b6363073b2ae9a7715b6686a8d6abe8d0390d838e4b9e6c201777ec1".to_string(),
            },
            staked_amount: 0,
            maturity: 0,
            dissolve_delay: 0,
            dissolving: false,
        };

        assert_eq!(result, expeted_result)
    }
}
