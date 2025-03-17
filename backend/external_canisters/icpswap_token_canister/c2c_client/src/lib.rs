use canister_client::generate_candid_c2c_call;

pub mod getToken {
    use icpswap_token_canister::PublicTokenOverview;

    use super::*;
    pub type Args = String;
    pub type Response = PublicTokenOverview;
}

generate_candid_c2c_call!(getToken);
