use candid::{self, CandidType, Deserialize};
use ic_cdk::{query, update};

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[query]
fn icrc10_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-10/ICRC-10.md".to_string(),
            name: "ICRC-10".to_string(),
        },
        SupportedStandard {
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/icrc_28_trusted_origins.md".to_string(),
            name: "ICRC-28".to_string(),
        },
    ]
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>,
}

// list every base URL that users will authenticate to your app from
#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("https://gldt.org"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.icp0.io"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.raw.icp0.io"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.ic0.app"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.raw.ic0.app"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.icp0.icp-api.io"),
        String::from("https://qfyvj-4qaaa-aaaam-qbfka-cai.icp-api.io"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.icp0.io"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.raw.icp0.io"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.ic0.app"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.raw.ic0.app"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.icp0.icp-api.io"),
        String::from("https://wmd7w-pqaaa-aaaai-qplaa-cai.icp-api.io"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.icp0.io"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.raw.icp0.io"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.ic0.app"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.raw.ic0.app"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.icp0.icp-api.io"),
        String::from("https://oj7ri-2qaaa-aaaap-abrzq-cai.icp-api.io"),
    ];

    return Icrc28TrustedOriginsResponse { trusted_origins };
}
