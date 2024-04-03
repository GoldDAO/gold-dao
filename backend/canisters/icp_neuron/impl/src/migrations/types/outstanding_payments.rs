use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{ Deserialize, Serialize };

#[derive(Default, Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPaymentsList(Vec<(u64, OutstandingPayments)>);

#[derive(Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPayments(pub Vec<OutstandingPayment>);

#[derive(Deserialize, Serialize, CandidType, Clone)]
pub struct OutstandingPayment {
    pub to: Account,
    pub amount: u64,
    pub status: PaymentStatus,
}

#[derive(Deserialize, Serialize, CandidType, Clone, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Complete,
}
