use candid::Principal;

pub type Args = (Principal, Principal, u64);
pub type Response = Result<String, String>;
