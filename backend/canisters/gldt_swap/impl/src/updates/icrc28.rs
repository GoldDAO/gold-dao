use candid::{self, CandidType, Deserialize};
use ic_cdk::update;

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

    Icrc28TrustedOriginsResponse { trusted_origins }
}
