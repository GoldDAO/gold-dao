use std::{ borrow::Cow, collections::BTreeMap };

use candid::{ CandidType, Decode, Encode, Nat, Principal };
use canister_time::now_millis;
use icrc_ledger_types::icrc1::account::Subaccount;
use num_bigint::BigUint;
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use tracing::debug;
use types::{ NeuronInfo, TimestampMillis, TokenInfo, TokenSymbol };
use ic_stable_structures::{ storable::Bound, StableBTreeMap, Storable };

use crate::memory::{ get_payment_round_history_memory, VM };
const MAX_VALUE_SIZE: u32 = 1000000;

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
        let mut next_key = match self.round_history.last_key_value() {
            Some(((_, id), _)) => {
                if id == 0 { 1 } else { id + 1 }
            } // Add 1 to the last key
            None => 1, // If the map is empty, start from 0
        };

        if next_key == u16::MAX {
            next_key = 1; // Wrap around to 0 if the key exceeds u16::MAX
        }
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

// ********************************
//    Payment Rounds
// ********************************

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct PaymentRound {
    pub id: u16, // id of the round. must start at 1 and will go to 65,535 before cycling to 1. Can't be 0 because 0 is the id of the reward pool accounts
    pub round_funds_total: Nat, // total amount to be distributed from the funds sub account
    pub tokens_to_distribute: Nat,
    pub fees: Nat, // total fees required for all valid transactions
    pub ledger_id: Principal, // the ledger associated with transferring funds for this round of specific token payments
    pub token: TokenSymbol, // the token associated with a specific payment round
    pub date_initialized: TimestampMillis, //
    pub total_neuron_maturity: u64, // total maturity of all neurons for this specific period
    pub payments: BTreeMap<NeuronId, Payment>, // map of payments to process
    pub retries: u8,
}

pub type RewardShare = Nat;
pub type MaturityDelta = u64;
pub type Payment = (RewardShare, PaymentStatus, MaturityDelta);

impl PaymentRound {
    pub fn new(
        id: u16,
        reward_pool_balance: Nat,
        token_info: TokenInfo,
        token: TokenSymbol,
        neuron_data: BTreeMap<NeuronId, NeuronInfo>
    ) -> Result<Self, String> {
        // we must pay a single fee to transfer from the reward pool to the round reward pool
        let neuron_maturity_for_interval = Self::calculate_neuron_maturity_for_interval(
            &neuron_data,
            &token
        )?;

        let total_neuron_maturity_for_interval = Self::calculate_aggregated_maturity(
            &neuron_maturity_for_interval
        );
        if token_info.fee > reward_pool_balance.clone() {
            let err = format!(
                "ROUND ID : {} & TOKEN : {:?} - Can't create PaymentRound. it would cost more than the balance of the reward pool to send it to the round pool",
                id,
                token.clone()
            );
            return Err(err);
        }
        // 2_000_000 - 10_000; = 1_990_000
        let round_funds_total = reward_pool_balance.clone() - Nat::from(token_info.fee);
        // 100_000
        let transaction_fees = Self::calculate_transaction_fees(
            &neuron_maturity_for_interval,
            token_info.fee
        )?;

        if transaction_fees > round_funds_total.clone() {
            let err = format!(
                "ROUND ID : {} & TOKEN : {:?} - Can't create PaymentRound. The fees for all payments in the payment round exceed the amount in the reward pool. distribution will inevitably result in some transactions containing insufficient funds",
                id,
                token.clone()
            );
            return Err(err);
        }
        let tokens_to_distribute = round_funds_total.clone() - transaction_fees.clone();

        if total_neuron_maturity_for_interval == 0u64 {
            let err = format!(
                "ROUND ID : {} & TOKEN : {:?} - Can't create PaymentRound. Maturity for all neurons has not changed since last distribution - exiting distribution early",
                id,
                token.clone()
            );
            return Err(err);
        }

        // rewards per neuron
        let payments = Self::calculate_neuron_rewards(
            neuron_maturity_for_interval,
            tokens_to_distribute.clone()
        )?;

        Ok(Self {
            id: id,
            round_funds_total: round_funds_total,
            tokens_to_distribute,
            fees: transaction_fees,
            ledger_id: token_info.ledger_id,
            token,
            date_initialized: now_millis(),
            total_neuron_maturity: total_neuron_maturity_for_interval,
            payments,
            retries: 0,
        })
    }

    pub fn calculate_neuron_maturity_for_interval(
        neurons: &BTreeMap<NeuronId, NeuronInfo>,
        token: &TokenSymbol
    ) -> Result<Vec<(NeuronId, u64)>, String> {
        // creates a list of the NeuronIds and their respective change in maturity.
        // A comparison is done against the rewarded_maturity ( maturity that has been already distributed to )
        // in order to get difference in maturity.
        let (neuron_maturity_ok, neuron_maturity_err): (Vec<_>, Vec<_>) = neurons
            .iter()
            .map(|(neuron_id, neuron_info)| {
                let previous_rewarded = neuron_info.rewarded_maturity
                    .get(token)
                    .copied()
                    .unwrap_or(0);
                let accumulated = neuron_info.accumulated_maturity;
                let delta_maturity = accumulated.checked_sub(previous_rewarded);
                (neuron_id.clone(), delta_maturity)
            })
            .partition(|&(_, mat)| mat.is_some());

        if neuron_maturity_err.is_empty() {
            let neuron_maturity_ok: Vec<_> = neuron_maturity_ok
                .into_iter()
                .map(|(neuron_id, maturity)| (neuron_id, maturity.unwrap()))
                .collect();
            println!(" = : {:?}", neuron_maturity_ok);

            Ok(neuron_maturity_ok)
        } else {
            Err("failed to calculate all neuron maturity for interval".to_string())
        }
    }

    pub fn calculate_transaction_fees(
        neuron_maturity_deltas: &[(NeuronId, u64)],
        single_fee: u64
    ) -> Result<Nat, String> {
        // get only the neurons that have a positive maturity delta/change.
        let neurons_with_positive_maturity_delta: Vec<&(NeuronId, u64)> = neuron_maturity_deltas
            .iter()
            .filter(|(_, maturity)| *maturity > 0u64)
            .collect();

        let number_of_valid_transactions = neurons_with_positive_maturity_delta.len() as u64;
        let total_fees = number_of_valid_transactions.checked_mul(single_fee);

        match total_fees {
            Some(total) => Ok(Nat::from(total)),
            None => Err("overflow when calculating total fees".to_string()),
        }
    }

    pub fn calculate_aggregated_maturity(data: &[(NeuronId, u64)]) -> u64 {
        data.iter()
            .map(|entry| entry.1)
            .sum()
    }

    // in order to preserve high precision division we use integer division.
    // in order to use integer division we need to scale some of our numbers using BigUint.
    // calculates the percentage of a reward each neuron should get.
    pub fn calculate_neuron_rewards(
        neuron_deltas: Vec<(NeuronId, u64)>,
        reward_pool: Nat
    ) -> Result<BTreeMap<NeuronId, Payment>, String> {
        let total_maturity: u64 = neuron_deltas
            .iter()
            .map(|entry| entry.1)
            .sum();

        let total_maturity_big = BigUint::from(total_maturity);
        let scale_factor = BigUint::from(100_000_000_000_000u64);

        // return early if 0 - prevent dividing error
        if total_maturity == 0u64 {
            return Err("No change in maturity - skipping round".to_string());
        }

        let reward_pool_big = BigUint::from(reward_pool);

        // calculates percentage using integer division ( scales numbers and descales )
        let map: BTreeMap<NeuronId, Payment> = neuron_deltas
            .iter()
            .map(|(neuron_id, maturity)| {
                // Convert maturity to BigUint
                let maturity_big = BigUint::from(*maturity);
                // Calculate percentage as (maturity / total_maturity) * scaling factor ( for extra precision )
                let percentage = (maturity_big * &scale_factor) / total_maturity_big.clone();
                let reward = (reward_pool_big.clone() * percentage) / &scale_factor;
                // return a new Payment
                (neuron_id.clone(), (Nat::from(reward), PaymentStatus::Pending, *maturity))
            })
            .filter(|(_, (reward, _, _))| reward.clone() > 0u64)
            .collect();

        Ok(map)
    }

    /// converts a u16 to a valid sub account
    /// payment round sub accounts should always have their ids at the END of a 32 byte array of 0's
    pub fn get_payment_round_sub_account_id(&self) -> Subaccount {
        let mut subaccount: [u8; 32] = [0; 32];
        // u16 -> bytes
        let num_bytes: [u8; 2] = self.id.to_be_bytes();
        // add u16 bytes to end of 32 byte array
        subaccount[32 - 2..].copy_from_slice(&num_bytes);

        subaccount
    }
}

#[derive(Serialize, Deserialize, CandidType, PartialEq, Eq, Debug, Clone)]
pub enum PaymentRoundStatus {
    Pending,
    InProgress,
    CompletedFull, // all payments completed successfully
    CompletedPartial, // some payments completed and some failed
    Failed(String),
}

#[derive(Serialize, Deserialize, CandidType, PartialEq, Eq, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use std::collections::{ BTreeMap, HashMap };

    use candid::Nat;
    use sns_governance_canister::types::NeuronId;
    use types::{ NeuronInfo, TokenSymbol };

    use crate::model::payment_processor::PaymentRound;

    #[test]
    fn test_calculate_neuron_rewards_scenario_1() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_4 = NeuronId::new(
            "4b9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_5 = NeuronId::new(
            "3b9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_6 = NeuronId::new(
            "4c9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_7 = NeuronId::new(
            "5a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_8 = NeuronId::new(
            "6a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_9 = NeuronId::new(
            "7a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_10 = NeuronId::new(
            "8a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_11 = NeuronId::new(
            "9a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_12 = NeuronId::new(
            "109ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_13 = NeuronId::new(
            "11ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98c"
        ).unwrap();
        let neuron_id_14 = NeuronId::new(
            "129ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_15 = NeuronId::new(
            "139ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_16 = NeuronId::new(
            "149ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_17 = NeuronId::new(
            "159ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_18 = NeuronId::new(
            "169ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_19 = NeuronId::new(
            "179ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_20 = NeuronId::new(
            "189ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        // normal example, we'd have a lower amount of overal maturity filled by many smaller values and so the distance between the max and min is a smaller distribution of delta.
        let neuron_deltas = vec![
            (neuron_id_1, 10000u64),
            (neuron_id_2, 300u64),
            (neuron_id_3, 200u64),
            (neuron_id_4, 10000u64),
            (neuron_id_5, 300u64),
            (neuron_id_6, 200u64),
            (neuron_id_7, 10000u64),
            (neuron_id_8, 300u64),
            (neuron_id_9, 200u64),
            (neuron_id_10, 10000u64),
            (neuron_id_11, 300u64),
            (neuron_id_12, 200u64),
            (neuron_id_13, 10000u64),
            (neuron_id_14, 4u64),
            (neuron_id_15, 200u64),
            (neuron_id_16, 10000u64),
            (neuron_id_17, 300u64),
            (neuron_id_18, 200u64),
            (neuron_id_19, 10000u64),
            (neuron_id_20, 300u64)
        ];
        let reward_pool = Nat::from(33_300_000_000u64); // 333 ICP
        let expected: Vec<u64> = vec![
            91_227_877, // if 1ICP = 20 USD then rough 0.9 ICP ~ 18 USD
            4_561_393_896, // roughly 45 ICP
            1_824_557, // 0.01 ICP - 0.20 USD for holding quite small amount
            91_227_877,
            4_561_393_896,
            136_841_816,
            91_227_877,
            4_561_393_896,
            136_841_816,
            4_561_393_896,
            136_841_816,
            136_841_816,
            91_227_877,
            4_561_393_896,
            91_227_877,
            4_561_393_896,
            136_841_816,
            91_227_877,
            4_561_393_896,
            136_841_816
        ];

        let result = PaymentRound::calculate_neuron_rewards(neuron_deltas, reward_pool).unwrap();
        println!("{:?}", result);
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(res, expected_value)| {
                assert_eq!(&res.1.0, expected_value);
            });
        // assert_eq!(true, false)
    }

    #[test]
    fn test_calculate_neuron_rewards_scenario_2() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        // extreme example, some neurons have a lot of maturity, some are new. this means we're forced to generate large and small rewards - more representative of when there is a large gap in the time since neuron creation
        let neuron_deltas = vec![
            (neuron_id_1, 20000000u64),
            (neuron_id_2, 10u64),
            (neuron_id_3, 10u64)
        ];
        let reward_pool = Nat::from(33_300_000_000u64); // 333 ICP
        let expected: Vec<u64> = vec![33_299_966_700u64, 16649u64, 16649u64];

        let result = PaymentRound::calculate_neuron_rewards(neuron_deltas, reward_pool).unwrap();
        println!("{:?}", result);
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(res, expected_value)| {
                assert_eq!(&res.1.0, expected_value);
            });
        // assert_eq!(true, false)
    }

    #[test]
    fn test_calculate_neuron_rewards_all_zeros() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 0u64), (neuron_id_3, 0u64)];
        let reward_pool = Nat::from(100_000_000u64); // 1 ICP

        let result = PaymentRound::calculate_neuron_rewards(neuron_deltas, reward_pool).is_err();
        assert_eq!(result, true)
    }

    #[test]
    fn test_calculate_neuron_rewards_with_no_maturity_change() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 30u64), (neuron_id_3, 30u64)];
        let reward_pool = Nat::from(100_000_000u64); // 1 ICP
        let expected: Vec<u64> = vec![50_000_000u64, 50_000_000u64];

        let result = PaymentRound::calculate_neuron_rewards(neuron_deltas, reward_pool).unwrap();
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(res, expected_value)| {
                assert_eq!(&res.1.0, expected_value);
            });
    }

    #[test]
    fn test_calculate_neuron_maturity_for_interval() {
        let mut neurons = BTreeMap::new();

        // neuron 1
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut neuron_1_rewarded = HashMap::new();
        let icp_symbol = TokenSymbol::parse("ICP").unwrap();
        neuron_1_rewarded.insert(icp_symbol.clone(), 0);

        let neuron_info_1 = NeuronInfo {
            accumulated_maturity: 150,
            last_synced_maturity: 150,
            rewarded_maturity: neuron_1_rewarded,
        };
        neurons.insert(neuron_id_1.clone(), neuron_info_1);
        let result = PaymentRound::calculate_neuron_maturity_for_interval(
            &neurons,
            &icp_symbol
        ).unwrap();
        let expected = 150;
        assert_eq!(result[0].1, expected);

        // simulate paying the user

        // payout previous maturity ( 150 ) && update the neuron maturity ( simulate new neuron maturity data )
        let n = neurons.get_mut(&neuron_id_1).unwrap();
        n.accumulated_maturity = 542;
        n.last_synced_maturity = 542;
        let rewarded_mat = n.rewarded_maturity.get_mut(&icp_symbol).unwrap();
        *rewarded_mat += 150;
        let result = PaymentRound::calculate_neuron_maturity_for_interval(
            &neurons,
            &icp_symbol
        ).unwrap();
        let expected = 392; // 542 (current maturity) - 150 (previous maturity)
        assert_eq!(result[0].1, expected);
    }

    #[test]
    fn test_calculate_neuron_maturity_for_interval_all_zeros() {
        let mut neurons = BTreeMap::new();
        let icp_symbol = TokenSymbol::parse("ICP").unwrap();

        // neuron 1
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let mut neuron_1_rewarded = HashMap::new();
        neuron_1_rewarded.insert(icp_symbol.clone(), 0);

        let neuron_info_1 = NeuronInfo {
            accumulated_maturity: 0,
            last_synced_maturity: 0,
            rewarded_maturity: neuron_1_rewarded,
        };
        neurons.insert(neuron_id_1.clone(), neuron_info_1);

        let result = PaymentRound::calculate_neuron_maturity_for_interval(
            &neurons,
            &icp_symbol
        ).unwrap();
        let expected = 0;
        assert_eq!(result[0].1, expected);
    }

    #[test]
    fn test_calculate_aggregated_maturity() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 10u64), (neuron_id_2, 20u64), (neuron_id_3, 30u64)];
        let res = PaymentRound::calculate_aggregated_maturity(&neuron_deltas);
        let expected = 60u64;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_calculate_transaction_fees() {
        let neuron_id_1 = NeuronId::new(
            "2a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_2 = NeuronId::new(
            "3a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();
        let neuron_id_3 = NeuronId::new(
            "4a9ab729b173e14cc88c6c4d7f7e9f3e7468e72fc2b49f76a6d4f5af37397f98"
        ).unwrap();

        let neuron_deltas = vec![(neuron_id_1, 0u64), (neuron_id_2, 30u64), (neuron_id_3, 30u64)];
        let expected = Nat::from(20_000u64); // 2 x neurons with positive maturity

        let result = PaymentRound::calculate_transaction_fees(&neuron_deltas, 10_000u64).unwrap();
        assert_eq!(result, expected);
    }
}
