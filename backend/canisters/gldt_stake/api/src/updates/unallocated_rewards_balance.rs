use candid::Nat;
use gldt_stake_common::manage_stake_position_interface::GeneralError;
use std::collections::HashMap;
use types::TokenSymbol;

pub type Args = ();
pub type Response = HashMap<TokenSymbol, Result<Nat, GeneralError>>;
