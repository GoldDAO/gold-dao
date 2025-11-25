use bity_ic_canister_time::timestamp_seconds;
use bity_ic_icrc3::transaction::TransactionType;
use candid::{CandidType, Nat};
use gldt_swap_api_archive::swap::SwapInfo as SwapInfoOld;
use gldt_swap_common::swap::SwapInfo as SwapInfoNew;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::TimestampSeconds;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SwapTransaction {
    pub btype: String,
    pub timestamp: u64,
    pub tx: SwapTransactionData,
}

impl SwapTransaction {
    pub fn new(swap_info: SwapInfoNew) -> Self {
        let btype = swap_info.swap_type.to_string();
        Self {
            btype,
            timestamp: timestamp_seconds(),
            tx: SwapTransactionData::New(swap_info),
        }
    }

    pub fn migrate(swap_info: SwapInfoOld) -> Self {
        let btype = match &swap_info {
            SwapInfoOld::Forward(_) => "forward_swap_old".to_string(),
            SwapInfoOld::Reverse(_) => "reverse_swap_old".to_string(),
        };
        Self {
            btype,
            timestamp: timestamp_seconds(),
            tx: SwapTransactionData::Old(swap_info),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SwapTransactionData {
    New(SwapInfoNew),
    Old(SwapInfoOld),
}

impl From<SwapTransaction> for ICRC3Value {
    fn from(tx: SwapTransaction) -> Self {
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

impl From<SwapTransactionData> for ICRC3Value {
    fn from(data: SwapTransactionData) -> Self {
        match data {
            SwapTransactionData::New(swap_info) => swap_info.into(),
            SwapTransactionData::Old(swap_info) => ICRC3Value::from(swap_info),
        }
    }
}

impl TransactionType for SwapTransaction {
    // TODO: add more validations
    fn validate_transaction_fields(&self) -> Result<(), String> {
        let swap_type_str = match &self.tx {
            SwapTransactionData::New(info) => info.swap_type.to_string(),
            SwapTransactionData::Old(info) => match info {
                SwapInfoOld::Forward(_) => "forward_swap_old".to_string(),
                SwapInfoOld::Reverse(_) => "reverse_swap_old".to_string(),
            },
        };

        if self.btype != swap_type_str {
            return Err("btype and swap_type must be the same".to_string());
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
