use candid::Principal;

pub type Args = Vec<Principal>;
pub type Response = Result<String, String>;
