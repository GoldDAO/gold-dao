use std::{ borrow::{ Borrow, BorrowMut, Cow }, collections::HashMap };

use candid::{ CandidType, Decode, Encode, Nat, Principal };
use canister_time::now_millis;
use futures::{ executor::block_on, future::join_all };
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::{ account::Account, transfer::TransferArg };
use serde::{ Deserialize, Serialize };
use sns_governance_canister::types::NeuronId;
use tracing::debug;
use types::{ TimestampMillis, TokenSymbol };
use ic_stable_structures::{ storable::Bound, StableBTreeMap, Storable };

use crate::memory::{ get_payment_round_history_memory, VM };
const MAX_VALUE_SIZE: u32 = 10000;

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
    pub fn next_key(&self) -> u16 {
        let mut next_key = match self.rounds.last_key_value() {
            Some((last_key, _)) => {
                if last_key == 0 { 1 } else { last_key + 1 }
            } // Add 1 to the last key
            None => 1, // If the map is empty, start from 0
        };

        if next_key > u16::MAX {
            next_key = 1; // Wrap around to 0 if the key exceeds u16::MAX
        }
        next_key
    }

    pub fn add_payment_round(&mut self, round: PaymentRound) {
        self.rounds.insert(round.id, round);
        debug!("New payment round created");
    }

    pub fn get_pending_payment_rounds(&mut self) -> Vec<(u16, PaymentRound)> {
        let rounds = self.rounds
            .iter()
            .filter(|round| round.1.round_status == PaymentRoundStatus::Pending)
            .map(|(round_id, payment_round)| (round_id, payment_round))
            .collect();

        rounds
    }

    pub fn read_pending_payment_rounds(&self) -> Vec<(u16, PaymentRound)> {
        let rounds = self.rounds
            .iter()
            .filter(|round| round.1.round_status == PaymentRoundStatus::Pending)
            .map(|(round_id, payment_round)| (round_id, payment_round))
            .collect();

        rounds
    }

    pub fn get_faulty_payment_rounds(&mut self) -> Vec<(u16, PaymentRound)> {
        let rounds = self.rounds
            .iter()
            .filter(|round| {
                match round.1.round_status {
                    PaymentRoundStatus::CompletedFull => false,
                    PaymentRoundStatus::Pending => false,
                    _ => true,
                }
            })
            .map(|(round_id, payment_round)| (round_id, payment_round))
            .collect();

        rounds
    }

    pub fn set_round_status(&mut self, id: &u16, status: PaymentRoundStatus) {
        let round = self.rounds.get(id);
        match round {
            Some(mut round) => {
                round.round_status = status;
            }
            None => {}
        }
    }

    pub fn set_payment_status(
        &mut self,
        round_id: &u16,
        neuron_id: &NeuronId,
        new_status: PaymentStatus
    ) {
        let round = self.rounds.get(round_id);
        match round {
            Some(mut round) => {
                round.payments.entry(neuron_id.clone()).and_modify(|(_, status, _)| {
                    *status = new_status;
                });
            }
            None => {}
        }
    }

    pub fn contains_faulty_payment_rounds(&self) -> bool {
        let rounds: Vec<(u16, PaymentRound)> = self.rounds
            .iter()
            .filter(|round| {
                match round.1.round_status {
                    PaymentRoundStatus::CompletedFull => false,
                    PaymentRoundStatus::Pending => false,
                    _ => true,
                }
            })
            .collect();

        return rounds.len() > 0;
    }

    pub fn process_pending_payment_rounds(
        &mut self
    ) -> Vec<(NeuronId, MaturityDelta, TokenSymbol)> {
        // only process pending rounds and only return successful payments
        let rounds_to_process = self.get_pending_payment_rounds();
        debug!(
            "rounds to process - processing pending payments for payment round id : {}",
            &rounds_to_process.len()
        );
        let mut finished: Vec<(NeuronId, MaturityDelta, TokenSymbol)> = vec![];
        for (round_id, mut payment_round) in rounds_to_process {
            debug!("Start - processing pending payments for payment round id : {}", round_id);

            let result = block_on(payment_round.start_payment_round());
            match result {
                Some(completed_payments) => {
                    let result: Vec<(NeuronId, MaturityDelta, TokenSymbol)> = completed_payments
                        .iter()
                        .map(|(neuron_id, (reward, status, maturity_delta))| (
                            neuron_id.clone(),
                            maturity_delta.clone(),
                            payment_round.token,
                        ))
                        .collect();

                    finished.extend(result);
                }
                None => {}
            }
        }
        finished
    }

    pub fn process_faulty_rounds(&mut self) -> Vec<(NeuronId, MaturityDelta, TokenSymbol)> {
        // only process rounds that fully failed, contain some failed or InProgress
        // TODO - maybe we can hook into when the canister is trapped, get all payment rounds that are InProgress and then switch the payment round status to interrupted. thereby avoiding
        // a slightly confusing need to process a InProgress payment.
        let rounds_to_process = self.get_faulty_payment_rounds();
        let mut finished: Vec<(NeuronId, MaturityDelta, TokenSymbol)> = vec![];
        for (round_id, mut payment_round) in rounds_to_process {
            let result = block_on(payment_round.start_payment_round());
            match result {
                Some(completed_payments) => {
                    let result: Vec<(NeuronId, MaturityDelta, TokenSymbol)> = completed_payments
                        .iter()
                        .map(|(neuron_id, (reward, status, maturity_delta))| (
                            neuron_id.clone(),
                            maturity_delta.clone(),
                            payment_round.token.clone(),
                        ))
                        .collect();

                    finished.extend(result);
                }
                None => {}
            }
        }
        finished
    }

    pub fn get_payment_rounds(&self) -> Vec<(u16, PaymentRound)> {
        debug!("1111111 logging rounds");
        let rounds = self.rounds
            .iter()
            .map(|(round_id, payment_round)| (round_id, payment_round))
            .collect();
        debug!("logging get_payment_rounds : {:?}", rounds);
        let first_round = self.rounds.get(&1);

        match first_round {
            Some(round) => {
                debug!("aaaa : {:?}", round);
            }
            None => {}
        }

        rounds
    }

    pub fn get_payment_round_by_id(&self, id: &u16) -> Option<PaymentRound> {
        self.rounds.get(id)
    }
}

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct PaymentRound {
    pub id: u16,
    pub round_funds_subaccount: Option<Subaccount>, // holds the rewards for this round of payments
    pub round_funds_total: Nat, // total amount to be distributed from the funds sub account
    pub ledger_id: Principal, // the ledger associated with transferring funds for this round of specific token payments
    pub token: TokenSymbol, // the token associated with a specific payment round
    pub date_initialized: TimestampMillis, //
    pub total_neuron_maturity: u64, // total maturity of all neurons for this specific period
    pub payments: HashMap<NeuronId, Payment>, // map of payments to process
    pub round_status: PaymentRoundStatus, // status of weather all payments passed, failed etc
}

pub type RewardShare = u64;
pub type MaturityDelta = u64;
pub type Payment = (RewardShare, PaymentStatus, MaturityDelta);

impl PaymentRound {
    pub fn new(
        id: u16,
        round_funds_total: Nat,
        ledger_id: Principal,
        token: TokenSymbol,
        total_neuron_maturity: u64,
        payments: HashMap<NeuronId, Payment>
    ) -> Self {
        Self {
            id: id,
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

    pub fn get_payment_round_sub_account_id(&self) -> Subaccount {
        let mut subaccount: [u8; 32] = [0; 32];

        // Convert the u16 number to bytes
        let num_bytes: [u8; 2] = self.id.to_be_bytes();

        // Copy the bytes of the u16 number to the end of the array
        subaccount[32 - 2..].copy_from_slice(&num_bytes);

        Subaccount(subaccount)
    }

    pub async fn start_payment_round(&mut self) -> Option<Vec<(NeuronId, Payment)>> {
        let batch_limit = 15; // 50 is the max but we do 3 transactions per neuron leaving 5 left ( 15 transactions x 3 token types)
        let round_pool_subaccount = match self.round_funds_subaccount {
            Some(value) => value,
            None => {
                self.round_status = PaymentRoundStatus::Failed(
                    "No subaccount for round pool found".to_string()
                );
                debug!("Fail - No sub account for payment round");
                return None;
            }
        };
        self.round_status = PaymentRoundStatus::InProgress;

        let ledger_id = self.ledger_id;

        let mut payments: Vec<(&NeuronId, &mut Payment)> = self
            .borrow_mut()
            .payments.iter_mut()
            .collect();

        let mut payments_chunks = payments.chunks_mut(batch_limit);

        while let Some(batch) = payments_chunks.next() {
            let transfer_futures = batch
                .iter_mut()
                .filter(|(_, (_, payment_status, _))| payment_status != &PaymentStatus::Completed)
                .map(|(neuron_id, (reward, payment_status, _))| {
                    let n_id = *neuron_id;
                    let account = Account {
                        owner: ic_cdk::api::id(),
                        subaccount: Some(n_id.into()),
                    };
                    *payment_status = PaymentStatus::Triggered;
                    transfer_token(round_pool_subaccount, account, ledger_id, Nat::from(*reward))
                });

            let results = join_all(transfer_futures).await;

            for (i, result) in results.into_iter().enumerate() {
                match result {
                    Ok(_) => {
                        batch[i].1.1 = PaymentStatus::Completed;
                    }
                    Err(e) => {
                        debug!("Transaction Failed - {}", e);
                        batch[i].1.1 = PaymentStatus::Failed(e);
                    }
                }
            }
        }
        // TODO - update the payment round status
        let finished_payments: Vec<(&NeuronId, &Payment)> = self.payments
            .borrow()
            .into_iter()
            .collect();

        let payment_round_status = determine_payment_round_post_status(finished_payments);
        self.round_status = payment_round_status;

        let only_successful_payments: Vec<(NeuronId, Payment)> = self.payments
            .borrow()
            .into_iter()
            .filter(|(_, (_, status, _))| status == &PaymentStatus::Completed)
            .map(|(neuron_id, (reward, status, maturity))| (
                neuron_id.clone(),
                (reward.clone(), status.clone(), maturity.clone()),
            ))
            .collect();
        Some(only_successful_payments)
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

async fn transfer_token(
    from_sub_account: Subaccount,
    to_account: Account,
    ledger_id: Principal,
    amount: Nat
) -> Result<(), String> {
    match
        icrc_ledger_canister_c2c_client::icrc1_transfer(
            ledger_id,
            &(TransferArg {
                from_subaccount: Some(from_sub_account.0),
                to: to_account,
                fee: Some((10_000u32).into()),
                created_at_time: None,
                amount: amount,
                memo: None,
            })
        ).await
    {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(error)) => Err(format!("Transfer error: {error:?}")),
        Err(error) => Err(format!("Network error: {error:?}")),
    }
}

fn determine_payment_round_post_status(
    payment_statuses: Vec<(&NeuronId, &Payment)>
) -> PaymentRoundStatus {
    let mut completed_count = 0;
    let mut failed_count = 0;

    for (_, (_, payment_status, _)) in &payment_statuses {
        match payment_status {
            PaymentStatus::Completed => {
                completed_count += 1;
            }
            PaymentStatus::Failed(_) => {
                failed_count += 1;
            }
            _ => {} // Ignore other statuses
        }
    }

    if completed_count > 0 && failed_count > 0 {
        PaymentRoundStatus::CompletedPartial
    } else if completed_count == payment_statuses.len() {
        PaymentRoundStatus::CompletedFull
    } else {
        PaymentRoundStatus::Failed("All payments for payment round failed".to_string())
    }
}
