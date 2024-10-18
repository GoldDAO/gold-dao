use canister_client::{ generate_c2c_call, generate_candid_c2c_call };
use gldt_swap_common::swap::{ SwapId, SwapInfo };
use gldt_swap_api_archive::get_archive_swaps::{
    Args as GetArchiveSwapsArgs,
    Response as GetArchiveSwapsResponse,
};
use gldt_swap_api_archive::get_archive_size::{
    Args as GetArchiveSizeArgs,
    Response as GetArchiveSizeResponse,
};
pub use gldt_swap_api_archive::get_swap_indexes_for_user::{
    Args as GetSwapIndexesForUserArgs,
    Response as GetSwapIndexesForUserResponse,
};

pub use gldt_swap_api_archive::get_swap_bulk::{
    Args as GetArchiveSwapBulkArgs,
    Response as GetArchiveSwapBulkResponse,
};

pub use gldt_swap_api_archive::archive_swap::{
    Args as ArchiveSwapArgs,
    Response as ArchiveSwapResponse,
};

pub use gldt_swap_api_archive::get_archive_swap::{
    Args as GetArchiveSwapArgs,
    Response as GetArchiveSwapResponse,
};

pub mod archive_swap {
    use super::*;
    pub type Args = ArchiveSwapArgs;
    pub type Response = ArchiveSwapResponse;
}

generate_candid_c2c_call!(archive_swap);

pub mod get_archive_swap {
    use super::*;
    pub type Args = GetArchiveSwapArgs;
    pub type Response = GetArchiveSwapResponse;
}

generate_candid_c2c_call!(get_archive_swap);

pub mod get_archive_swaps {
    use super::*;
    pub type Args = GetArchiveSwapsArgs;
    pub type Response = GetArchiveSwapsResponse;
}

generate_candid_c2c_call!(get_archive_swaps);

pub mod get_archive_size {
    use super::*;
    pub type Args = GetArchiveSizeArgs;
    pub type Response = GetArchiveSizeResponse;
}

generate_candid_c2c_call!(get_archive_size);

pub mod get_swap_indexes_for_user {
    use super::*;
    pub type Args = GetSwapIndexesForUserArgs;
    pub type Response = GetSwapIndexesForUserResponse;
}

generate_candid_c2c_call!(get_swap_indexes_for_user);

pub mod get_swap_bulk {
    use super::*;
    pub type Args = GetArchiveSwapBulkArgs;
    pub type Response = GetArchiveSwapBulkResponse;
}

generate_candid_c2c_call!(get_swap_bulk);
