use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use types::NnsNeuronId;

#[derive(Default, Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub struct OutstandingPaymentsList(HashMap<NnsNeuronId, PaymentsList>);

impl OutstandingPaymentsList {
    pub fn get_outstanding_payments(&self, neuron_id: NnsNeuronId) -> Option<PaymentsList> {
        self.0.get(&neuron_id).cloned()
    }

    pub fn remove_from_list(&mut self, neuron_id: NnsNeuronId) {
        self.0.remove(&neuron_id);
    }

    pub fn cleanup(&mut self) {
        // removes any leftover outstanding payments which are complete, in case they were missed before
        let mut keys_to_remove = vec![];
        self.0.iter().for_each(|(&key, val)| {
            if val.all_complete() {
                keys_to_remove.push(key)
            }
        });
        keys_to_remove
            .iter()
            .for_each(|&neuron_id| self.remove_from_list(neuron_id))
    }

    pub fn insert(
        &mut self,
        neuron_id: NnsNeuronId,
        payment: PaymentsList,
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
        status: PaymentStatus,
    ) {
        if let Some(entry) = self.0.get_mut(&neuron_id) {
            entry.update_status(account, status)
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq, Debug)]
pub struct PaymentsList {
    #[serde_as(as = "Vec<(_, _)>")]
    // needed to have it corrected serialised for the /metrics http endpoint
    pub list: HashMap<Account, Payment>,
}

impl PaymentsList {
    pub fn new(list: Vec<(Account, u64)>) -> Self {
        let map: HashMap<Account, Payment> = list
            .into_iter()
            .map(|(account, amount)| (account, Payment::new(amount)))
            .collect();
        Self { list: map }
    }
    pub fn all_complete(&self) -> bool {
        self.list.iter().all(|(_, payment)| payment.is_complete())
    }
    pub fn has_some(&self) -> bool {
        !self.list.is_empty()
    }
    pub fn has_none(&self) -> bool {
        !self.has_some()
    }

    pub fn update_status(&mut self, account: Account, status: PaymentStatus) {
        if let Some(payment) = self.list.get_mut(&account) {
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
#[cfg(test)]
mod tests {
    use crate::testing::dummies::dummy_account;

    use super::*;

    #[test]
    fn test_get_outstanding_payments() {
        let mut list = OutstandingPaymentsList::default();
        let neuron_id = 1;
        let payments = PaymentsList::new(vec![(dummy_account(None), 100)]);
        let _ = list.insert(neuron_id, payments.clone());

        assert_eq!(list.get_outstanding_payments(neuron_id), Some(payments));
        assert_eq!(list.get_outstanding_payments(2), None);
    }

    #[test]
    fn test_remove_from_list() {
        let mut list = OutstandingPaymentsList::default();
        let neuron_id = 1;
        let payments = PaymentsList::new(vec![(dummy_account(None), 100)]);
        let _ = list.insert(neuron_id, payments.clone());

        list.remove_from_list(neuron_id);
        assert_eq!(list.get_outstanding_payments(neuron_id), None);
    }

    #[test]
    fn test_update_status_of_entry_in_list() {
        let mut list = OutstandingPaymentsList::default();
        let neuron_id = 1;
        let account = dummy_account(None);
        let payments = PaymentsList::new(vec![(account.clone(), 100)]);
        let _ = list.insert(neuron_id, payments.clone());

        assert_eq!(
            list.get_outstanding_payments(neuron_id).unwrap().list[&account].status,
            PaymentStatus::Pending
        );

        list.update_status_of_entry_in_list(neuron_id, account.clone(), PaymentStatus::Complete);

        assert_eq!(
            list.get_outstanding_payments(neuron_id).unwrap().list[&account].status,
            PaymentStatus::Complete
        );
    }

    #[test]
    fn test_new_outstanding_payments() {
        let account1 = dummy_account(None);
        let account2 = dummy_account(Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));
        let list = PaymentsList::new(vec![(account1.clone(), 100), (account2.clone(), 200)]);

        assert_eq!(list.list[&account1].amount, 100);
        assert_eq!(list.list[&account1].status, PaymentStatus::Pending);
        assert_eq!(list.list[&account2].amount, 200);
        assert_eq!(list.list[&account2].status, PaymentStatus::Pending);
    }

    #[test]
    fn test_all_complete() {
        let account1 = dummy_account(None);
        let account2 = dummy_account(Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));
        let mut list = PaymentsList::new(vec![(account1.clone(), 100), (account2.clone(), 200)]);

        assert_eq!(list.all_complete(), false);

        list.update_status(account1.clone(), PaymentStatus::Complete);
        assert_eq!(list.all_complete(), false);

        list.update_status(account2.clone(), PaymentStatus::Complete);
        assert_eq!(list.all_complete(), true);
    }

    #[test]
    fn test_has_some() {
        let account1 = dummy_account(None);
        let account2 = dummy_account(Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));
        let list = PaymentsList::new(vec![(account1.clone(), 100), (account2.clone(), 200)]);

        assert_eq!(list.has_some(), true);
    }

    #[test]
    fn test_has_none() {
        let list = PaymentsList::new(vec![]);

        assert_eq!(list.has_none(), true);
    }

    #[test]
    fn test_update_status() {
        let account1 = dummy_account(None);
        let account2 = dummy_account(Some([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));
        let mut list = PaymentsList::new(vec![(account1.clone(), 100), (account2.clone(), 200)]);

        list.update_status(account1.clone(), PaymentStatus::Complete);
        assert_eq!(list.list[&account1].status, PaymentStatus::Complete);

        list.update_status(account2.clone(), PaymentStatus::Pending);
        assert_eq!(list.list[&account2].status, PaymentStatus::Pending);
    }
}
