#![allow(non_snake_case)]
use canister_client::generate_candid_c2c_call;

pub mod getToken {
    use icpswap_token_canister::PublicTokenOverview;

    pub type Args = String;
    pub type Response = PublicTokenOverview;
}

generate_candid_c2c_call!(getToken);
