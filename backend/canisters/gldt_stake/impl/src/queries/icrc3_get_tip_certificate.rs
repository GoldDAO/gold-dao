use crate::state::icrc3_get_tip_certificate as icrc3_get_tip_certificate_impl;
pub use gldt_stake_api_canister::icrc3_get_tip_certificate::{
    Args as GetTipCertificateArg, Response as GetTipCertificateResponse,
};
use ic_cdk::query;

#[query]
async fn icrc3_get_tip_certificate(_: GetTipCertificateArg) -> GetTipCertificateResponse {
    icrc3_get_tip_certificate_impl()
}
