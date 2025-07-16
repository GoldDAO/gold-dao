use bity_ic_canister_time::timestamp_millis;
use bity_ic_icrc3::transaction::TransactionType;
use candid::CandidType;
use candid::Nat;
use candid::Principal;
use gldt_stake_common::stake_position_event::DissolveStakeEvent;
use ic_cdk::api::msg_caller;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use types::TimestampSeconds;
use types::TokenSymbol;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EventTransaction {
    pub btype: String,
    pub timestamp: u64,
    pub tx: EventTransactionData,
}

impl EventTransaction {
    pub fn new(call: StakePositionStateChange) -> Self {
        let op = match &call {
            StakePositionStateChange::AddStake { .. } => "add_stake",
            StakePositionStateChange::ClaimRewards { .. } => "claim_rewards",
            StakePositionStateChange::StartDissolving { .. } => "start_dissolving",
            StakePositionStateChange::DissolveInstantly { .. } => "dissolve_instantly",
            StakePositionStateChange::Withdraw { .. } => "withdraw",
        };
        let timestamp = timestamp_millis();
        Self {
            btype: op.to_string(),
            timestamp,
            tx: EventTransactionData {
                op: op.to_string(),
                caller: msg_caller(),
                stake_position_change: call,
                created_at_time: Some(timestamp.into()),
            },
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EventTransactionData {
    pub op: String, // need to be == to btype
    pub created_at_time: Option<Nat>,
    pub stake_position_change: StakePositionStateChange,
    pub caller: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum StakePositionStateChange {
    AddStake {
        amount_added: Nat,
        result_staked: Nat,
    },
    ClaimRewards {
        rewards_before_claim: Vec<(TokenSymbol, Nat)>,
        reward_updates: Vec<(TokenSymbol, Nat)>,
    },
    StartDissolving {
        fraction: u8,
        amount_dissolved: Nat,
        result_staked: Nat,
        dissolve_events: Vec<DissolveStakeEvent>,
    },
    DissolveInstantly {
        fraction: u8,
        amount_dissolved: Nat,
        result_staked: Nat,
    },
    Withdraw {
        dissolved_events: Vec<DissolveStakeEvent>,
        amount_withdrawn: Nat,
    },
}

impl From<EventTransaction> for ICRC3Value {
    fn from(tx: EventTransaction) -> Self {
        let mut map = BTreeMap::new();
        map.insert("btype".to_string(), ICRC3Value::Text(tx.btype));
        map.insert(
            "timestamp".to_string(),
            ICRC3Value::Nat(Nat::from(tx.timestamp)),
        );
        map.insert("tx".to_string(), ICRC3Value::from(tx.tx));
        ICRC3Value::Map(map)
    }
}

impl From<EventTransactionData> for ICRC3Value {
    fn from(data: EventTransactionData) -> Self {
        let mut map = BTreeMap::new();
        map.insert("op".to_string(), ICRC3Value::Text(data.op));
        map.insert(
            "caller".to_string(),
            ICRC3Value::Text(data.caller.to_text()),
        );
        map.insert(
            "created_at_time".to_string(),
            match data.created_at_time {
                Some(n) => ICRC3Value::Nat(n),
                None => ICRC3Value::Text("None".to_string()),
            },
        );
        ICRC3Value::Map(map)
    }
}

impl TransactionType for EventTransaction {
    fn validate_transaction_fields(&self) -> Result<(), String> {
        if self.btype != self.tx.op {
            return Err("btype and op must be the same".to_string());
        }
        Ok(())
    }

    fn timestamp(&self) -> Option<TimestampSeconds> {
        Some(self.timestamp)
    }

    fn tx(&self) -> ICRC3Value {
        self.tx.clone().into()
    }

    fn block_type(&self) -> String {
        self.btype.clone()
    }
}
