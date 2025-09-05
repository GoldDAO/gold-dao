use bity_ic_canister_time::timestamp_seconds;
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
        let timestamp = timestamp_seconds();
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

impl Into<ICRC3Value> for StakePositionStateChange {
    fn into(self) -> ICRC3Value {
        let mut map = BTreeMap::new();
        match self {
            StakePositionStateChange::AddStake {
                amount_added,
                result_staked,
            } => {
                map.insert("amount_added".to_string(), ICRC3Value::Nat(amount_added));
                map.insert("result_staked".to_string(), ICRC3Value::Nat(result_staked));
            }
            StakePositionStateChange::ClaimRewards {
                rewards_before_claim,
                reward_updates,
            } => {
                let rewards_before: Vec<_> = rewards_before_claim
                    .into_iter()
                    .map(|(sym, amt)| {
                        let mut inner = BTreeMap::new();
                        inner.insert("token".to_string(), ICRC3Value::Text(sym.to_string()));
                        inner.insert("amount".to_string(), ICRC3Value::Nat(amt));
                        ICRC3Value::Map(inner)
                    })
                    .collect();

                let rewards_after: Vec<_> = reward_updates
                    .into_iter()
                    .map(|(sym, amt)| {
                        let mut inner = BTreeMap::new();
                        inner.insert("token".to_string(), ICRC3Value::Text(sym.to_string()));
                        inner.insert("amount".to_string(), ICRC3Value::Nat(amt));
                        ICRC3Value::Map(inner)
                    })
                    .collect();

                map.insert(
                    "rewards_before_claim".to_string(),
                    ICRC3Value::Array(rewards_before),
                );
                map.insert(
                    "reward_updates".to_string(),
                    ICRC3Value::Array(rewards_after),
                );
            }
            StakePositionStateChange::StartDissolving {
                fraction,
                amount_dissolved,
                result_staked,
                dissolve_events,
            } => {
                map.insert("fraction".to_string(), ICRC3Value::Nat(fraction.into()));
                map.insert(
                    "amount_dissolved".to_string(),
                    ICRC3Value::Nat(amount_dissolved),
                );
                map.insert("result_staked".to_string(), ICRC3Value::Nat(result_staked));

                let events: Vec<_> = dissolve_events
                    .into_iter()
                    .map(|e| e.into()) // requires Into<ICRC3Value> for DissolveStakeEvent
                    .collect();
                map.insert("dissolve_events".to_string(), ICRC3Value::Array(events));
            }
            StakePositionStateChange::DissolveInstantly {
                fraction,
                amount_dissolved,
                result_staked,
            } => {
                map.insert("fraction".to_string(), ICRC3Value::Nat(fraction.into()));
                map.insert(
                    "amount_dissolved".to_string(),
                    ICRC3Value::Nat(amount_dissolved),
                );
                map.insert("result_staked".to_string(), ICRC3Value::Nat(result_staked));
            }
            StakePositionStateChange::Withdraw {
                dissolved_events,
                amount_withdrawn,
            } => {
                map.insert(
                    "amount_withdrawn".to_string(),
                    ICRC3Value::Nat(amount_withdrawn),
                );

                let events: Vec<_> = dissolved_events
                    .into_iter()
                    .map(|e| e.into()) // requires Into<ICRC3Value> for DissolveStakeEvent
                    .collect();
                map.insert("dissolved_events".to_string(), ICRC3Value::Array(events));
            }
        }
        ICRC3Value::Map(map)
    }
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
            "stake_position_change".to_string(),
            data.stake_position_change.into(),
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
