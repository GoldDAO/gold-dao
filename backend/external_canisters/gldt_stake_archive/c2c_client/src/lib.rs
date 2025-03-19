use canister_client::{generate_c2c_call, generate_candid_c2c_call};
use gldt_stake_api_archive::get_archive_size::{
    Args as GetArchiveSizeArgs, Response as GetArchiveSizeResponse,
};
pub use gldt_stake_api_archive::get_item_indexes_for_user::{
    Args as GetItemIndexesForUserArgs, Response as GetItemIndexesForUserResponse,
};

pub use gldt_stake_api_archive::get_item_bulk::{
    Args as GetArchiveItemBulkArgs, Response as GetArchiveItemBulkResponse,
};

pub use gldt_stake_api_archive::archive_item::{
    Args as ArchiveItemArgs, Response as ArchiveItemResponse,
};

pub use gldt_stake_api_archive::get_archive_item::{
    Args as GetArchiveItemArgs, Response as GetArchiveItemResponse,
};

pub mod archive_item {
    use super::*;
    pub type Args = ArchiveItemArgs;
    pub type Response = ArchiveItemResponse;
}

generate_candid_c2c_call!(archive_item);

pub mod get_archive_item {
    use super::*;
    pub type Args = GetArchiveItemArgs;
    pub type Response = GetArchiveItemResponse;
}

generate_candid_c2c_call!(get_archive_item);

pub mod get_archive_size {
    use super::*;
    pub type Args = GetArchiveSizeArgs;
    pub type Response = GetArchiveSizeResponse;
}

generate_candid_c2c_call!(get_archive_size);

pub mod get_item_indexes_for_user {
    use super::*;
    pub type Args = GetItemIndexesForUserArgs;
    pub type Response = GetItemIndexesForUserResponse;
}

generate_candid_c2c_call!(get_item_indexes_for_user);

pub mod get_item_bulk {
    use super::*;
    pub type Args = GetArchiveItemBulkArgs;
    pub type Response = GetArchiveItemBulkResponse;
}

generate_candid_c2c_call!(get_item_bulk);
