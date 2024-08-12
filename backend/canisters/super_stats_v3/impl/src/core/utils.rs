use candid::{Principal, Nat};
use ic_cdk::api::call::RejectionCode;
use num_traits::ToPrimitive;
use super_stats_v3_api::{core::constants::{OC_DEV_BOT, OC_DEV_PRINCIPAL}, runtime::RUNTIME_STATE};

pub fn log(text: impl AsRef<str>){
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.add_log(text.as_ref().to_string())
    });
}

pub async fn canister_call<T, U> (canister: &str, method: &str, args: T) -> Result<(U,), (RejectionCode, String)> 
where
    T: candid::CandidType,
    U: for<'a> candid::Deserialize<'a, > + candid::CandidType,
{
    // for T: use your struct directly
    // for U: use result Result<(YourResponseType,), (ic_cdk::api::call::RejectionCode, String)>
    
    let canister_id = Principal::from_text(canister);
    match canister_id {
        Ok(pr) => {
            let call:Result<(U,), (RejectionCode, String)> = 
            ic_cdk::call(pr, method, ( args,)).await;
            match call {
                Ok(value) => { Ok(value)}
                Err(e) => {
                    Err(e)
                }
            }
        },
        Err(e) => { 
            let er = format!("Could not parse canister principal {:?}", e);
            return Err((ic_cdk::api::call::RejectionCode::Unknown, er))
        }
    }
}

pub async fn oc_dev_message(message: String){
    // OC-Bot Canister
    let can = Principal::from_text(OC_DEV_BOT);
    match can {
        Ok(cnster) => {
            // Dev OC A/C
            let user = String::from(OC_DEV_PRINCIPAL);
            let _res: Result<((),), (ic_cdk::api::call::RejectionCode, String)> 
            = ic_cdk::call(cnster, "send", (user, message, )).await;
        },
        Err(_error) => {}
    }
}

pub async fn critical_err(message: String){
    let canister = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let error = format!("{} - Canister: {}",message, canister);
    log(error.clone());
    oc_dev_message(error).await;
}

pub fn nat_to_u128(nat: Nat) -> Result<u128, String> {
   match nat.0.to_u128(){
        Some(v) => {
            return Ok(v);
        },
        None => {
            return Err("Not a valid u128".to_string())
        }
   }
}

pub fn nat_to_u64(nat: Nat) -> Result<u64, String> {
    match nat.0.to_u64(){
         Some(v) => {
             return Ok(v);
         },
         None => {
             return Err("Not a valid u64".to_string())
         }
    }
 }