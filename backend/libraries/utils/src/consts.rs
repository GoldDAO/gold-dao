use candid::Principal;
use types::CanisterId;

pub const E8S_PER_ICP: u64 = 100_000_000;

pub const SNS_ROOT_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 124, 1, 1]
);
pub const SNS_GOVERNANCE_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 125, 1, 1]
);
pub const STAGING_SNS_GOVERNANCE_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 224, 14, 183, 1, 1]
);
pub const SNS_LEDGER_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 126, 1, 1]
);
pub const SNS_LEDGER_INDEX_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 2, 0, 0, 128, 1, 1]
);
pub const NNS_GOVERNANCE_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1]
);
pub const ICP_LEDGER_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 0, 0, 0, 2, 1, 1]
);
pub const CYCLES_MINTING_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 0, 0, 0, 4, 1, 1]
);
pub const GOLD_1G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 80, 16, 81, 1, 1]
);
pub const STAGING_GOLD_1G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 80, 17, 132, 1, 1]
);
pub const GOLD_10G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 192, 69, 198, 1, 1]
);
pub const STAGING_GOLD_10G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 112, 15, 122, 1, 1]
);
pub const GOLD_100G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 96, 17, 146, 1, 1]
);
pub const GOLD_1000G_CANISTER_ID: CanisterId = Principal::from_slice(
    &[0, 0, 0, 0, 1, 128, 10, 88, 1, 1]
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
    fn staging_sns_governance_canister_id() {
        assert_eq!(
            STAGING_SNS_GOVERNANCE_CANISTER_ID,
            Principal::from_text("j3ioe-7iaaa-aaaap-ab23q-cai").unwrap()
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

    #[test]
    fn icp_ledger_canister_id() {
        assert_eq!(
            ICP_LEDGER_CANISTER_ID,
            Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
        );
    }

    #[test]
    fn nns_governance_canister_id() {
        assert_eq!(
            NNS_GOVERNANCE_CANISTER_ID,
            Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
        );
    }

    #[test]
    fn cycles_minting_canister_id() {
        assert_eq!(
            CYCLES_MINTING_CANISTER_ID,
            Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
        );
    }

    #[test]
    fn gold_1g_canister_id() {
        assert_eq!(
            GOLD_1G_CANISTER_ID,
            Principal::from_text("io7gn-vyaaa-aaaak-qcbiq-cai").unwrap()
        );
    }

    #[test]
    fn gold_staging_1g_canister_id() {
        assert_eq!(
            STAGING_GOLD_1G_CANISTER_ID,
            Principal::from_text("obapm-2iaaa-aaaak-qcgca-cai").unwrap()
        );
    }


    #[test]
    fn gold_10g_canister_id() {
        assert_eq!(
            GOLD_10G_CANISTER_ID,
            Principal::from_text("sy3ra-iqaaa-aaaao-aixda-cai").unwrap()
        );
    }

    #[test]
    fn gold_staging_10g_canister_id() {
        assert_eq!(
            STAGING_GOLD_10G_CANISTER_ID,
            Principal::from_text("xyo2o-gyaaa-aaaal-qb55a-cai").unwrap()
        );
    }

    #[test]
    fn gold_100g_canister_id() {
        assert_eq!(
            GOLD_100G_CANISTER_ID,
            Principal::from_text("zhfjc-liaaa-aaaal-acgja-cai").unwrap()
        );
    }

    #[test]
    fn gold_1000g_canister_id() {
        assert_eq!(
            GOLD_1000G_CANISTER_ID,
            Principal::from_text("7i7jl-6qaaa-aaaam-abjma-cai").unwrap()
        );
    }
}
