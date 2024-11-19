use canister_client::generate_candid_c2c_call;

use origyn_nft_reference::origyn_nft_reference_canister::Account3;
use origyn_nft_reference::origyn_nft_reference_canister::NftInfoResult;
use origyn_nft_reference::origyn_nft_reference_canister::{ManageSaleRequest, ManageSaleResult};
use origyn_nft_reference::origyn_nft_reference_canister::{
    MarketTransferRequest, MarketTransferResult,
};
use origyn_nft_reference::origyn_nft_reference_canister::{SaleInfoRequest, SaleInfoResult};
use origyn_nft_reference::origyn_nft_reference_canister::{TransferArgs, TransferResult};

use candid::Nat;

pub mod nft_origyn {
    // use origyn_nft_reference::origyn_nft_reference_canister::NftInfoResult;

    use super::*;
    pub type Args = String;
    pub type Response = NftInfoResult;
}
pub mod icrc7_transfer {
    // use origyn_nft_reference::origyn_nft_reference_canister::NftInfoResult;

    use super::*;
    pub type Args = Vec<TransferArgs>;
    pub type Response = TransferResult;
}
pub mod get_token_id_as_nat {
    use super::*;
    pub type Args = String;
    pub type Response = Nat;
}
pub mod icrc7_transfer_fee {
    use super::*;
    pub type Args = Nat;
    pub type Response = Option<Nat>;
}

pub mod get_nat_as_token_id_origyn {
    use super::*;
    pub type Args = Nat;
    pub type Response = String;
}

pub mod icrc7_owner_of {
    use super::*;
    pub type Args = Vec<Nat>;
    pub type Response = Vec<Option<Account3>>;
}

pub mod sale_info_nft_origyn {
    use super::*;
    pub type Args = SaleInfoRequest;
    pub type Response = SaleInfoResult;
}

pub mod sale_nft_origyn {
    use super::*;
    pub type Args = ManageSaleRequest;
    pub type Response = ManageSaleResult;
}

pub mod market_transfer_nft_origyn {
    use super::*;
    pub type Args = MarketTransferRequest;
    pub type Response = MarketTransferResult;
}

pub mod icrc7_tokens_of {
    use super::*;
    pub type Args = Account3;
    pub type Response = Vec<candid::Nat>;
}

pub mod count_unlisted_tokens_of {
    use icrc_ledger_types::icrc1::account::Account;

    use super::*;
    pub type Args = Account;
    pub type Response = Nat;
}

generate_candid_c2c_call!(nft_origyn);
generate_candid_c2c_call!(icrc7_transfer);
generate_candid_c2c_call!(get_token_id_as_nat);
generate_candid_c2c_call!(icrc7_transfer_fee);
generate_candid_c2c_call!(get_nat_as_token_id_origyn);
generate_candid_c2c_call!(icrc7_owner_of);
generate_candid_c2c_call!(sale_info_nft_origyn);
generate_candid_c2c_call!(sale_nft_origyn);
generate_candid_c2c_call!(market_transfer_nft_origyn);
generate_candid_c2c_call!(icrc7_tokens_of);
generate_candid_c2c_call!(count_unlisted_tokens_of);
