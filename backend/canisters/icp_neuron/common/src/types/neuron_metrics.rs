use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use ledger_utils::icrc_account_to_legacy_account_id;
use nns_governance_canister::types::{neuron::DissolveState, Neuron};
use serde::Serialize;
use utils::consts::NNS_GOVERNANCE_CANISTER_ID;

#[derive(CandidType, Serialize, Debug, PartialEq, Eq)]
pub struct NeuronWithMetric {
    pub id: u64,
    pub deposit_account: Option<DepositAccount>,
    pub staked_amount: u64,
    pub maturity: u64,
    pub dissolve_delay: u64,
    pub dissolving: bool,
    pub voting_power_refreshed_timestamp_seconds: Option<u64>,
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
            Ok(bytes) => {
                let icrc_account = Account {
                    owner: NNS_GOVERNANCE_CANISTER_ID,
                    subaccount: Some(bytes),
                };
                let legacy_account_id = icrc_account_to_legacy_account_id(icrc_account).to_hex();
                Some(DepositAccount {
                    icrc_account,
                    legacy_account_id,
                })
            }
            Err(_) => None,
        };
        Self {
            id: neuron.id.map_or(0, |id| id.id),
            deposit_account,
            staked_amount: neuron.cached_neuron_stake_e8s,
            maturity: neuron.maturity_e8s_equivalent,
            dissolve_delay,
            dissolving,
            voting_power_refreshed_timestamp_seconds: neuron
                .voting_power_refreshed_timestamp_seconds,
        }
    }
}

#[derive(CandidType, Serialize, Debug, PartialEq, Eq)]
pub struct DepositAccount {
    icrc_account: Account,
    legacy_account_id: String,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use icrc_ledger_types::icrc1::account::Account;
    use nns_governance_canister::types::{Neuron, NeuronId};
    use utils::consts::NNS_GOVERNANCE_CANISTER_ID;

    use crate::types::neuron_metrics::DepositAccount;

    use super::NeuronWithMetric;

    #[test]
    fn convert_neuron_to_neuron_metric() {
        let neuron = Neuron {
            id: Some(NeuronId {
                id: 17_481_076_647_658_761_488,
            }),
            account: vec![
                149, 128, 178, 23, 182, 54, 48, 115, 178, 174, 154, 119, 21, 182, 104, 106, 141,
                106, 190, 141, 3, 144, 216, 56, 228, 185, 230, 194, 1, 119, 126, 193,
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
            deciding_voting_power: Some(0),
            neuron_type: Some(0),
            potential_voting_power: Some(0),
            visibility: Some(0),
            voting_power_refreshed_timestamp_seconds: Some(1234),
        };

        let result = NeuronWithMetric::from(neuron);

        let expeted_result = NeuronWithMetric {
            id: 17_481_076_647_658_761_488,
            deposit_account: Some(DepositAccount {
                icrc_account: Account {
                    owner: NNS_GOVERNANCE_CANISTER_ID,
                    subaccount: Some([
                        149, 128, 178, 23, 182, 54, 48, 115, 178, 174, 154, 119, 21, 182, 104, 106,
                        141, 106, 190, 141, 3, 144, 216, 56, 228, 185, 230, 194, 1, 119, 126, 193,
                    ]),
                },
                legacy_account_id:
                    "6601afb37d5807c9ed17c8343bb1c7180f98eca73a64727f56134c720cf0304a".to_string(),
            }),
            staked_amount: 0,
            maturity: 0,
            dissolve_delay: 0,
            dissolving: false,
            voting_power_refreshed_timestamp_seconds: Some(1234),
        };

        assert_eq!(result, expeted_result)
    }
}
