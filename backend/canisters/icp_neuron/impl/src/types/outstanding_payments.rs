use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{ Deserialize, Serialize };

#[derive(Default, Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPaymentsList(Vec<(u64, OutstandingPayments)>);

impl OutstandingPaymentsList {
    pub fn get_outstanding_payments(&self, neuron_id: u64) -> Option<OutstandingPayments> {
        self.0
            .iter()
            .find_map(|(id, payment)| if *id == neuron_id { Some(payment.clone()) } else { None })
    }

    pub fn settle(&mut self, neuron_id: u64) {
        if let Some(index) = self.0.iter().position(|&(id, _)| neuron_id == id) {
            self.0.remove(index);
        }
    }
    pub fn update(&mut self, neuron_id: u64, list: OutstandingPayments) {
        if let Some(index) = self.0.iter().position(|&(id, _)| neuron_id == id) {
            self.0[index].1 = list;
        } else {
            self.0.push((neuron_id, list));
        }
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPayments(pub Vec<OutstandingPayment>);

impl OutstandingPayments {
    pub fn new(list: Vec<(Account, u64)>) -> Self {
        Self(
            list
                .iter()
                .map(|(to, amount)| OutstandingPayment {
                    to: *to,
                    amount: *amount,
                    status: PaymentStatus::Pending,
                })
                .collect()
        )
    }
    pub fn all_settled(&self) -> bool {
        self.0.iter().all(|s| s.is_settled())
    }
    pub fn has_some(&self) -> bool {
        self.0.len() > 0
    }
    pub fn has_none(&self) -> bool {
        !self.has_some()
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPayment {
    pub to: Account,
    pub amount: u64,
    pub status: PaymentStatus,
}

impl OutstandingPayment {
    pub fn is_pending(&self) -> bool {
        self.status == PaymentStatus::Pending
    }
    pub fn is_settled(&self) -> bool {
        self.status == PaymentStatus::Complete
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Complete,
}
