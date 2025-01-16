use crate::numeric::{GoldPrice, GLDT, USDG};
use crate::vault::FeeBucket;
use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::account::Account;
use minicbor::{Decode, Encode};
use usdg_minter_api::types::{CandidEvent, CandidEventType};
use usdg_minter_api::ApiFeeBucket;

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub enum EventType {
    #[n(0)]
    Init {
        #[cbor(n(0), with = "crate::cbor::principal")]
        usdg_ledger_id: Principal,
        #[cbor(n(1), with = "crate::cbor::principal")]
        gldt_ledger_id: Principal,
        #[cbor(n(2), with = "crate::cbor::principal")]
        gold_dao_governance_id: Principal,
        #[cbor(n(3), with = "crate::cbor::principal")]
        xrc_id: Principal,
    },

    #[n(1)]
    Upgrade {
        #[n(0)]
        new_medium_fee_percent: Option<u64>,
    },

    #[n(2)]
    OpenVault {
        #[cbor(n(0), with = "crate::cbor::account")]
        owner: Account,
        #[n(1)]
        margin_amount: GLDT,
        #[n(2)]
        borrowed_amount: USDG,
        #[n(3)]
        fee_bucket: FeeBucket,
        #[n(4)]
        block_index: u64,
    },

    #[n(3)]
    Borrow {
        #[n(0)]
        vault_id: u64,
        #[n(1)]
        borrowed_amount: USDG,
        #[n(2)]
        block_index: u64,
    },

    #[n(4)]
    AddMargin {
        #[n(0)]
        vault_id: u64,
        #[n(1)]
        margin_added: GLDT,
        #[n(2)]
        block_index: u64,
    },

    #[n(5)]
    Repay {
        #[n(0)]
        vault_id: u64,
        #[n(1)]
        debt: USDG,
        #[n(2)]
        block_index: u64,
    },

    #[n(6)]
    Close {
        #[n(0)]
        vault_id: u64,
        #[n(1)]
        block_index: Option<u64>,
    },

    #[n(7)]
    TransferExecuted {
        #[n(0)]
        transfer_id: u64,
        #[n(1)]
        block_index: u64,
    },

    #[n(8)]
    DepositLiquidity {
        #[cbor(n(0), with = "crate::cbor::account")]
        caller: Account,
        #[n(1)]
        amount: USDG,
        #[n(2)]
        block_index: u64,
    },

    #[n(9)]
    WithdrawLiquidity {
        #[cbor(n(0), with = "crate::cbor::account")]
        caller: Account,
        #[n(1)]
        amount: USDG,
        #[n(2)]
        block_index: u64,
    },

    #[n(10)]
    ClaimReturns {
        #[cbor(n(0), with = "crate::cbor::account")]
        caller: Account,
        #[n(1)]
        amount: GLDT,
        #[n(2)]
        block_index: u64,
    },

    #[n(11)]
    Redeem {
        #[cbor(n(0), with = "crate::cbor::account")]
        owner: Account,
        #[n(1)]
        current_rate: GoldPrice,
        #[n(2)]
        amount: USDG,
        #[n(4)]
        block_index: u64,
    },

    #[n(12)]
    ChargeFee,

    #[n(13)]
    Liquidate {
        #[n(0)]
        vault_id: u64,
    },

    #[n(14)]
    Redistribute {
        #[n(0)]
        vault_id: u64,
    },

    #[n(15)]
    UpdateVault {
        #[n(0)]
        vault_id: u64,
        #[cbor(n(1), with = "crate::cbor::account::option")]
        new_owner: Option<Account>,
        #[n(2)]
        fee_bucket: Option<FeeBucket>,
    },
}

#[derive(CandidType, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct Event {
    /// The canister time at which the minter generated this event.
    #[n(0)]
    pub timestamp: u64,
    /// The event type.
    #[n(1)]
    pub payload: EventType,
}

pub fn event_to_candid_event(event: Event) -> CandidEvent {
    let event_type = match event.payload {
        EventType::Init {
            usdg_ledger_id,
            gldt_ledger_id,
            gold_dao_governance_id,
            xrc_id,
        } => CandidEventType::Init {
            usdg_ledger_id,
            gldt_ledger_id,
            gold_dao_governance_id,
            xrc_id,
        },
        EventType::Upgrade {
            new_medium_fee_percent,
        } => CandidEventType::Upgrade {
            new_medium_fee_percent,
        },
        EventType::OpenVault {
            owner,
            margin_amount,
            borrowed_amount,
            fee_bucket,
            block_index,
        } => {
            let api_fee_bucket = match fee_bucket {
                FeeBucket::Low => ApiFeeBucket::Low,
                FeeBucket::Medium => ApiFeeBucket::Medium,
                FeeBucket::High => ApiFeeBucket::High,
            };
            CandidEventType::OpenVault {
                owner,
                margin_amount: margin_amount.0,
                borrowed_amount: borrowed_amount.0,
                fee_bucket: api_fee_bucket,
                block_index,
            }
        }
        EventType::Borrow {
            vault_id,
            borrowed_amount,
            block_index,
        } => CandidEventType::Borrow {
            vault_id,
            borrowed_amount: borrowed_amount.0,
            block_index,
        },
        EventType::AddMargin {
            vault_id,
            margin_added,
            block_index,
        } => CandidEventType::AddMargin {
            vault_id,
            margin_added: margin_added.0,
            block_index,
        },
        EventType::Repay {
            vault_id,
            debt,
            block_index,
        } => CandidEventType::Repay {
            vault_id,
            debt: debt.0,
            block_index,
        },
        EventType::Close {
            vault_id,
            block_index,
        } => CandidEventType::Close {
            vault_id,
            block_index,
        },
        EventType::TransferExecuted {
            transfer_id,
            block_index,
        } => CandidEventType::TransferExecuted {
            transfer_id,
            block_index,
        },
        EventType::DepositLiquidity {
            caller,
            amount,
            block_index,
        } => CandidEventType::DepositLiquidity {
            caller,
            amount: amount.0,
            block_index,
        },
        EventType::WithdrawLiquidity {
            caller,
            amount,
            block_index,
        } => CandidEventType::WithdrawLiquidity {
            caller,
            amount: amount.0,
            block_index,
        },
        EventType::ClaimReturns {
            caller,
            amount,
            block_index,
        } => CandidEventType::ClaimReturns {
            caller,
            amount: amount.0,
            block_index,
        },
        EventType::Redeem {
            owner,
            current_rate,
            amount,
            block_index,
        } => CandidEventType::Redeem {
            owner,
            current_rate: current_rate.0,
            amount: amount.0,
            block_index,
        },
        EventType::ChargeFee => CandidEventType::ChargeFee,
        EventType::Liquidate { vault_id } => CandidEventType::Liquidate { vault_id },
        EventType::Redistribute { vault_id } => CandidEventType::Redistribute { vault_id },
        EventType::UpdateVault {
            vault_id,
            new_owner,
            fee_bucket,
        } => {
            let api_fee_bucket = match fee_bucket {
                Some(FeeBucket::Low) => Some(ApiFeeBucket::Low),
                Some(FeeBucket::Medium) => Some(ApiFeeBucket::Medium),
                Some(FeeBucket::High) => Some(ApiFeeBucket::High),
                None => None,
            };
            CandidEventType::UpdateVault {
                vault_id,
                new_owner,
                fee_bucket: api_fee_bucket,
            }
        }
    };
    CandidEvent {
        timestamp: event.timestamp,
        payload: event_type,
    }
}
