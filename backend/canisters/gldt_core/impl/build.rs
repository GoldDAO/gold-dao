// use ic_cdk_bindgen::{ Builder, Config };

/// activate this for debugging
// macro_rules! p {
//     ($($tokens:tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     };
// }

/// This functions sets the candid paths of the canisters.
/// This is needed because dfx for some reason doesn't automatically set those
fn set_env_vars() {
    std::env::set_var("CANISTER_CANDID_PATH_gld_nft", "../gld_nft/origyn_nft_reference.did");
    // std::env::set_var("CANISTER_CANDID_PATH_gld_nft", "../gld_nft/origyn_nft_reference.did");
    std::env::set_var("CANISTER_ID_gld_nft", "obapm-2iaaa-aaaak-qcgca-cai"); // dummy value
    // std::env::set_var(
    //     "CANISTER_CANDID_PATH_gldnft_backend_10g",
    //     "../gld_nft/origyn_nft_reference.did"
    // );
    // std::env::set_var(
    //     "CANISTER_CANDID_PATH_gldnft_backend_100g",
    //     "../gld_nft/origyn_nft_reference.did"
    // );
    // std::env::set_var(
    //     "CANISTER_CANDID_PATH_gldnft_backend_1000g",
    //     "../gld_nft/origyn_nft_reference.did"
    // );
    // std::env::set_var("CANISTER_CANDID_PATH_gldt_ledger", "../gldt_ledger/gldt_ledger.did");
}
pub fn main() {
    set_env_vars();
    // let mut builder = Builder::new();

    // let gld_nft = Config::new("gld_nft");
    // builder.add(gld_nft);

    // builder.build(None);
}
// succeeddd pipeline
