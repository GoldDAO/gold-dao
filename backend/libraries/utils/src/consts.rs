use candid::Principal;
use types::CanisterId;

pub const SNS_ROOT_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 124, 1, 1]
);
pub const SNS_GOVERNANCE_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 125, 1, 1]
);
pub const SNS_LEDGER_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 126, 1, 1]
);
pub const SNS_LEDGER_INDEX_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 128, 1, 1]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sns_root_canister_id() {
        assert_eq!(
            SNS_ROOT_CANISTER_ID,
            Principal::from_text("tw2vt-hqaaa-aaaaq-aab6a-cai").unwrap()
        );
    }

    #[test]
    fn sns_governance_canister_id() {
        assert_eq!(
            SNS_GOVERNANCE_CANISTER_ID,
            Principal::from_text("tr3th-kiaaa-aaaaq-aab6q-cai").unwrap()
        );
    }

    #[test]
    fn sns_ledger_canister_id() {
        assert_eq!(
            SNS_LEDGER_CANISTER_ID,
            Principal::from_text("tyyy3-4aaaa-aaaaq-aab7a-cai").unwrap()
        );
    }

    #[test]
    fn sns_ledger_index_canister_id() {
        assert_eq!(
            SNS_LEDGER_INDEX_CANISTER_ID,
            Principal::from_text("efv5g-kqaaa-aaaaq-aacaa-cai").unwrap()
        );
    }
}
