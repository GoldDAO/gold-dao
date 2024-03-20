use std::{ borrow::Cow, collections::HashMap };

use candid::{ CandidType, Decode, Encode, Nat, Principal };
use canister_time::now_millis;
use ic_ledger_types::Subaccount;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use types::{ TimestampMillis, Token };
use ic_stable_structures::{ storable::Bound, StableBTreeMap, Storable };

use crate::memory::{ get_payment_round_history_memory, VM };
const MAX_VALUE_SIZE: u32 = 1000;

/// The history of each neuron's maturity.
// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct PaymentProcessor {
    #[serde(skip, default = "init_map")]
    rounds: StableBTreeMap<u16, PaymentRound, VM>,
}

fn init_map() -> StableBTreeMap<u16, PaymentRound, VM> {
    let memory = get_payment_round_history_memory();
    StableBTreeMap::init(memory)
}

impl Default for PaymentProcessor {
    fn default() -> Self {
        Self { rounds: init_map() }
    }
}

fn create_payment_round_sub_account_id(count: u16) -> Subaccount {
    let u16_bytes: [u8; 2] = count.to_le_bytes();
    let mut array: [u8; 32] = [0; 32];
    array[30..32].copy_from_slice(&u16_bytes);
    Subaccount(array)
}

impl PaymentProcessor {
    pub fn add_payment_round(&mut self, round: PaymentRound) {
        let next_key = self.next_key();
        let mut new_round = round;
        new_round.round_funds_subaccount = Some(create_payment_round_sub_account_id(next_key));
        self.rounds.insert(next_key, new_round);
    }

    pub fn next_key(&self) -> u16 {
        let mut next_key = match self.rounds.last_key_value() {
            Some((last_key, _)) => last_key + 1, // Add 1 to the last key
            None => 0, // If the map is empty, start from 0
        };

        if next_key > u16::MAX {
            next_key = 0; // Wrap around to 0 if the key exceeds u16::MAX
        }
        next_key
    }
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct PaymentRound {
    pub round_funds_subaccount: Option<Subaccount>, // holds the rewards for this round of payments
    pub round_funds_total: Nat, // total amount to be distributed from the funds sub account
    pub ledger_id: Principal, // the ledger associated with transferring funds for this round of specific token payments
    pub token: Token, // the token associated with a specific payment round
    pub date_initialized: TimestampMillis, //
    pub total_neuron_maturity: u64, // total maturity of all neurons for this specific period
    pub payments: HashMap<NeuronId, (u64, PaymentStatus)>, // map of payments to process
    pub round_status: PaymentRoundStatus, // status of weather all payments passed, failed etc
}

impl PaymentRound {
    pub fn new(
        round_funds_total: Nat,
        ledger_id: Principal,
        token: Token,
        total_neuron_maturity: u64,
        payments: HashMap<NeuronId, (u64, PaymentStatus)>
    ) -> Self {
        Self {
            round_funds_subaccount: None,
            round_funds_total,
            ledger_id,
            token,
            date_initialized: now_millis(),
            total_neuron_maturity,
            payments,
            round_status: PaymentRoundStatus::Pending,
        }
    }
}

#[derive(Serialize, Deserialize, CandidType)]
pub enum PaymentRoundStatus {
    Pending,
    CompletedFull,
    CompletedPartial,
    InProgress,
    Failed,
}

#[derive(Serialize, Deserialize, CandidType)]
pub enum PaymentStatus {
    Pending,
    Triggered,
    Completed,
    Failed(String),
}

impl Storable for PaymentRound {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
