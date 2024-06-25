use std::collections::BTreeMap;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use sns_rewards_api_canister::payment_round::{ PaymentRound, PaymentStatus };
use tracing::debug;
use types::TokenSymbol;
use ic_stable_structures::StableBTreeMap;

use crate::memory::{ get_payment_round_history_memory, VM };

// ********************************
//    Payment Processor
// ********************************

// NOTE: Stable structures don't need to be serialized, hence the #[serde(skip)].
#[derive(Serialize, Deserialize)]
pub struct PaymentProcessor {
    /// Holds only PaymentRounds that are FULLY completed.
    #[serde(skip, default = "init_map")]
    round_history: StableBTreeMap<(TokenSymbol, u16), PaymentRound, VM>,
    /// Holds active PaymentRounds that are being processed
    active_rounds: BTreeMap<TokenSymbol, PaymentRound>,
}

fn init_map() -> StableBTreeMap<(TokenSymbol, u16), PaymentRound, VM> {
    let memory = get_payment_round_history_memory();
    StableBTreeMap::init(memory)
}
impl Default for PaymentProcessor {
    fn default() -> Self {
        Self {
            round_history: init_map(),
            active_rounds: BTreeMap::new(),
        }
    }
}

impl PaymentProcessor {
    // gets the last key of the last completed payment round and circles from 1 - u16::MAX - each cycle is 125 years.
    pub fn next_key(&self) -> u16 {
        let mut max_key = 0;
        for ((symbol, id), _) in self.round_history.iter() {
            if id > max_key {
                max_key = id;
            }
        }

        let next_key = if max_key == u16::MAX { 1 } else { max_key + 1 };
        next_key
    }

    pub fn add_active_payment_round(&mut self, round: PaymentRound) {
        self.active_rounds.insert(round.token.clone(), round);
    }

    pub fn get_active_rounds(&self) -> Vec<PaymentRound> {
        self.active_rounds
            .iter()
            .map(|(_, payment_round)| payment_round.clone())
            .collect()
    }

    pub fn set_active_payment_status(
        &mut self,
        round_token: &TokenSymbol,
        neuron_id: &NeuronId,
        new_status: PaymentStatus
    ) {
        if let Some(round) = self.active_rounds.get_mut(round_token) {
            if let Some(payment) = round.payments.get_mut(neuron_id) {
                payment.1 = new_status;
            } else {
                debug!(
                    "ERROR - ROUND ID : {} & TOKEN :{:?} - set_active_payment_status failed - can't find neuron id {:?}",
                    round.id,
                    round_token,
                    neuron_id
                );
            }
        } else {
            debug!(
                "ERROR - set_active_payment_status failed - can't find round {:?} in active_rounds",
                round_token
            );
        }
    }

    pub fn get_payment_round_history(
        &self,
        token: TokenSymbol,
        id: u16
    ) -> Vec<(u16, PaymentRound)> {
        let rounds = self.round_history
            .iter()
            .filter(|((_, round_id), round)| { *round_id == id && round.token == token })
            .map(|((_, round_id), payment_round)| (round_id, payment_round.clone()))
            .collect();

        rounds
    }

    pub fn add_to_history(&mut self, payment_round: PaymentRound) {
        self.round_history.insert((payment_round.token.clone(), payment_round.id), payment_round);
    }

    pub fn delete_active_round(&mut self, round_token: TokenSymbol) {
        self.active_rounds.remove_entry(&round_token);
    }

    pub fn set_payment_round_retry_count(&mut self, token: &TokenSymbol, attempt: u8) {
        if let Some(round) = self.active_rounds.get_mut(token) {
            round.retries = attempt;
        } else {
            debug!(
                "ERROR - set_payment_round_retry_count - can't find active round for token {:?}",
                token
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use candid::{ Nat, Principal };
    use sns_governance_canister::types::NeuronId;
    use sns_rewards_api_canister::payment_round::PaymentRound;
    use types::{ TimestampMillis, TokenSymbol };

    use crate::state::{ init_state, mutate_state, read_state, RuntimeState };

    fn init_runtime_state() {
        init_state(RuntimeState::default());
    }

    #[test]
    fn test_key_incrementation() {
        init_runtime_state();
        let neuron_id = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let icp_token = TokenSymbol::parse("ICP").unwrap();
        let ogy_token = TokenSymbol::parse("OGY").unwrap();
        let gldgov_token = TokenSymbol::parse("GLDGov").unwrap();

        mutate_state(|s|
            s.data.payment_processor.add_to_history(PaymentRound {
                id: 1,
                round_funds_total: Nat::from(100u64),
                tokens_to_distribute: Nat::from(100u64),
                fees: Nat::from(100u64),
                ledger_id: Principal::anonymous(),
                token: icp_token.clone(),
                date_initialized: 1,
                total_neuron_maturity: 100,
                payments: BTreeMap::default(),
                retries: 0,
            })
        );
        mutate_state(|s|
            s.data.payment_processor.add_to_history(PaymentRound {
                id: 1,
                round_funds_total: Nat::from(100u64),
                tokens_to_distribute: Nat::from(100u64),
                fees: Nat::from(100u64),
                ledger_id: Principal::anonymous(),
                token: ogy_token.clone(),
                date_initialized: 1,
                total_neuron_maturity: 100,
                payments: BTreeMap::default(),
                retries: 0,
            })
        );

        read_state(|s| {
            assert_eq!(s.data.payment_processor.next_key(), 2);
        });

        mutate_state(|s|
            s.data.payment_processor.add_to_history(PaymentRound {
                id: 2,
                round_funds_total: Nat::from(100u64),
                tokens_to_distribute: Nat::from(100u64),
                fees: Nat::from(100u64),
                ledger_id: Principal::anonymous(),
                token: icp_token.clone(),
                date_initialized: 200,
                total_neuron_maturity: 100,
                payments: BTreeMap::default(),
                retries: 0,
            })
        );

        read_state(|s| {
            assert_eq!(s.data.payment_processor.next_key(), 3);
        });

        mutate_state(|s|
            s.data.payment_processor.add_to_history(PaymentRound {
                id: 3,
                round_funds_total: Nat::from(100u64),
                tokens_to_distribute: Nat::from(100u64),
                fees: Nat::from(100u64),
                ledger_id: Principal::anonymous(),
                token: icp_token.clone(),
                date_initialized: 1,
                total_neuron_maturity: 100,
                payments: BTreeMap::default(),
                retries: 0,
            })
        );
        mutate_state(|s|
            s.data.payment_processor.add_to_history(PaymentRound {
                id: 3,
                round_funds_total: Nat::from(100u64),
                tokens_to_distribute: Nat::from(100u64),
                fees: Nat::from(100u64),
                ledger_id: Principal::anonymous(),
                token: ogy_token.clone(),
                date_initialized: 1,
                total_neuron_maturity: 100,
                payments: BTreeMap::default(),
                retries: 0,
            })
        );

        read_state(|s| {
            assert_eq!(s.data.payment_processor.next_key(), 4);
        });
    }
}
