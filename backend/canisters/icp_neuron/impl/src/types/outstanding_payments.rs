use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{ Deserialize, Serialize };
use types::NnsNeuronId;
use std::collections::HashMap;

#[derive(Default, Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub struct OutstandingPaymentsList(HashMap<NnsNeuronId, PaymentsList>);

impl OutstandingPaymentsList {
    pub fn get_outstanding_payments(&self, neuron_id: NnsNeuronId) -> Option<PaymentsList> {
        self.0.get(&neuron_id).cloned()
    }

    pub fn remove_from_list(&mut self, neuron_id: NnsNeuronId) {
        self.0.remove(&neuron_id);
    }

    pub fn insert(
        &mut self,
        neuron_id: NnsNeuronId,
        payment: PaymentsList
    ) -> Result<(), PaymentsList> {
        if let Some(payment) = self.0.get(&neuron_id) {
            Err(payment.clone())
        } else {
            self.0.insert(neuron_id, payment);
            Ok(())
        }
    }

    pub fn update_status_of_entry_in_list(
        &mut self,
        neuron_id: NnsNeuronId,
        account: Account,
        status: PaymentStatus
    ) {
        match self.0.get_mut(&neuron_id) {
            Some(entry) => entry.update_status(account, status),
            None => (),
        }
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub struct PaymentsList(pub HashMap<Account, Payment>);

impl PaymentsList {
    pub fn new(list: Vec<(Account, u64)>) -> Self {
        let map: HashMap<Account, Payment> = list
            .into_iter()
            .map(|(account, amount)| { (account, Payment::new(amount)) })
            .collect();
        Self(map)
    }
    pub fn all_complete(&self) -> bool {
        self.0.iter().all(|(_, payment)| payment.is_complete())
    }
    pub fn has_some(&self) -> bool {
        self.0.len() > 0
    }
    pub fn has_none(&self) -> bool {
        !self.has_some()
    }

    pub fn update_status(&mut self, account: Account, status: PaymentStatus) {
        if let Some(payment) = self.0.get_mut(&account) {
            payment.update_status(status)
        }
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub struct Payment {
    amount: u64,
    status: PaymentStatus,
}

impl Payment {
    pub fn new(amount: u64) -> Self {
        Self {
            amount,
            status: PaymentStatus::Pending,
        }
    }
    pub fn update_status(&mut self, status: PaymentStatus) {
        self.status = status;
    }
    pub fn is_complete(&self) -> bool {
        self.status == PaymentStatus::Complete
    }
    pub fn get_amount(&self) -> u64 {
        self.amount
    }
}

#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub enum PaymentStatus {
    Pending,
    Complete,
}
