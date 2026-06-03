use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use utils::consts::NNS_GOVERNANCE_CANISTER_ID;

pub const ICP_NEURON_SUBACCOUNT: Subaccount = [
    79, 96, 141, 115, 220, 220, 191, 84, 127, 214, 63, 137, 4, 23, 221, 213, 136, 242, 149, 58, 68,
    204, 159, 74, 162, 218, 161, 12, 161, 104, 39, 29,
];

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StakeIcpConfig {
    // Account to transfer ICP to when neither swap conditions are met.
    pub destination_account: Account,
    /// The NNS neuron ID to top up. Used to call ClaimOrRefresh after the transfer.
    pub neuron_id: u64,
    pub rate_per_interval: u64,
    pub job_interval_ms: u64,
    pub min_amount: u64,
}

impl Default for StakeIcpConfig {
    fn default() -> Self {
        Self {
            destination_account: Account {
                owner: NNS_GOVERNANCE_CANISTER_ID,
                subaccount: Some(ICP_NEURON_SUBACCOUNT),
            },
            neuron_id: 7446549063176501841,
            rate_per_interval: 2_380_950, // 1/42 of the week
            job_interval_ms: Duration::from_secs(14400).as_millis() as u64,
            min_amount: 10_000_000, // 0.1 ICP
        }
    }
}

impl StakeIcpConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.job_interval_ms < 60_000 {
            return Err("job_interval_ms must be at least 60_000".to_string());
        }
        if self.rate_per_interval == 0 {
            return Err("rate_per_interval must be greater than 0".to_string());
        }
        if self.min_amount == 0 {
            return Err("min_amount must be greater than 0".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use utils::principal::PrincipalDotAccountFormat;

    use super::*;

    #[test]
    fn test_destination_account() {
        let acc = Account {
            owner: NNS_GOVERNANCE_CANISTER_ID,
            subaccount: Some(ICP_NEURON_SUBACCOUNT),
        };
        println!("Destination account: {:?}", acc.to_principal_dot_account());
    }
}
