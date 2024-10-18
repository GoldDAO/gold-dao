use candid::{ Nat, Principal };
use canister_time::timestamp_millis;
use gldt_swap_common::{
    gldt::{ GldtNumTokens, GLDT_PRICE_RATIO, GLDT_SUBDIVIDABLE_BY },
    nft::{ NftCanisterConf, NftID, NftWeight },
    swap::{
        NftValidationError,
        SwapDetailForward,
        SwapDetailReverse,
        SwapErrorReverse,
        SwapInfo,
        SwapStatusForward,
        SwapStatusReverse,
        SwapType,
    },
};

use gldt_swap_api_canister::{
    swap_nft_for_tokens::NftInvalidError,
    swap_tokens_for_nft::Args as SwapTokensForNftArgs,
};

pub use gldt_swap_api_canister::notify_sale_nft_origyn::Args as SubscriberNotification;
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_reference::origyn_nft_reference_canister::Account3;
use origyn_nft_reference_c2c_client::{
    get_nat_as_token_id_origyn,
    get_token_id_as_nat,
    icrc7_owner_of,
};
use types::TimestampMillis;
use utils::{ env::Environment, retry_async::retry_async };
use crate::{ state::{ read_state, FeeAccount }, utils::trace };

#[derive(Clone)]
pub struct SwapBuilder<T> {
    _marker: std::marker::PhantomData<T>, // Keeps the type parameter without needing a field
}

impl SwapBuilder<SwapDetailForward> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
    fn _verify_nft_weight(
        &self,
        prin: &Principal
    ) -> Result<(Principal, NftCanisterConf, Option<FeeAccount>), NftInvalidError> {
        let gldnft_canisters = read_state(|s| s.data.gldnft_canisters.clone());

        match gldnft_canisters.iter().find(|(principal, ..)| principal == prin) {
            Some(conf) => Ok(conf.clone()),
            None => Err(NftInvalidError::InvalidNFTCollectionPrincipal),
        }
    }

    pub async fn _get_nft_nat_id(
        &self,
        nft_id: &String,
        nft_canister_id: Principal
    ) -> Result<Nat, NftInvalidError> {
        match get_token_id_as_nat(nft_canister_id, &nft_id.into()).await {
            Ok(id) => Ok(id),
            Err(_) => { Err(NftInvalidError::CantGetNatIdOfNft) }
        }
    }

    fn _calculate_tokens_from_weight(
        &self,
        grams: &NftWeight
    ) -> Result<GldtNumTokens, NftInvalidError> {
        match
            GldtNumTokens::new(
                Nat::from(*grams) * Nat::from(GLDT_PRICE_RATIO) * GLDT_SUBDIVIDABLE_BY
            )
        {
            Ok(tokens) => Ok(tokens),
            Err(_) => Err(NftInvalidError::InvalidTokenAmount),
        }
    }

    pub async fn _get_origyn_id_string(
        &self,
        nft_id: &NftID,
        nft_canister_id: &Principal
    ) -> Result<String, NftInvalidError> {
        match
            retry_async(|| get_nat_as_token_id_origyn(nft_canister_id.clone(), &nft_id.0), 3).await
        {
            Ok(id) => {
                return Ok(id);
            }
            Err((rejection_code, msg)) => {
                let msg = format!("{rejection_code:?}. {msg}");
                return Err(
                    NftInvalidError::CantGetOrigynID(
                        format!("Can't get origyn ID for NFT nat : {nft_id:?}. error = {msg}")
                    )
                );
            }
        }
    }

    pub async fn _check_nft_owner(
        &self,
        nft_canister_id: &Principal,
        nft_id: &NftID,
        user_principal: &Principal
    ) -> Result<(), NftInvalidError> {
        match
            retry_async(|| icrc7_owner_of(nft_canister_id.clone(), vec![nft_id.0.clone()]), 3).await
        {
            Ok(result) => {
                match &result[0] {
                    Some(account) => {
                        if &account.owner != user_principal {
                            let nft_owner = account.owner;
                            return Err(
                                NftInvalidError::InvalidNftOwner(
                                    format!(
                                        "Nft is owned by : {nft_owner}. but swap has owner of : {user_principal}"
                                    )
                                )
                            );
                        } else {
                            return Ok(());
                        }
                    }
                    None => {
                        return Err(
                            NftInvalidError::InvalidNftOwner(
                                format!(
                                    "can't verify NFT ownership. Nft with nat id : {nft_id:?} not found"
                                )
                            )
                        );
                    }
                }
            }
            Err((rejection_code, msg)) => {
                return Err(
                    NftInvalidError::InvalidNftOwner(
                        format!(
                            "can't verify NFT ownership. call failed. {rejection_code:?} - msg = {msg}"
                        )
                    )
                );
            }
        };
    }

    pub async fn init(
        self,
        nft_id: NftID,
        nft_canister_id: Principal,
        time: TimestampMillis,
        user_principal: Principal
    ) -> Result<SwapInfo, (SwapInfo, Vec<NftInvalidError>)> {
        let mut new_swap = SwapInfo::new(SwapType::Forward);
        let mut errors: Vec<NftInvalidError> = vec![];

        if let Err(e) = &self._check_nft_owner(&nft_canister_id, &nft_id, &user_principal).await {
            errors.push(e.clone());
        }

        let nft_id_string = match &self._get_origyn_id_string(&nft_id, &nft_canister_id).await {
            Ok(id) => id.clone(),
            Err(e) => {
                errors.push(e.clone());
                "InvalidId".to_string()
            }
        };

        let (_, weight, _) = match self._verify_nft_weight(&nft_canister_id) {
            Ok(conf) => conf,
            Err(e) => {
                errors.push(e);
                (Principal::anonymous(), NftCanisterConf { grams: 0 }, None)
            }
        };

        let tokens_to_mint = match self._calculate_tokens_from_weight(&weight.grams) {
            Ok(num_tokens) => num_tokens,
            Err(e) => {
                errors.push(e);
                GldtNumTokens::invalid()
            }
        };

        trace(&format!("//////// {tokens_to_mint:?}"));

        let is_locked = read_state(|s| s.data.swaps.get_active_swap_by_string_id(&nft_id_string));
        if is_locked.is_some() {
            errors.push(NftInvalidError::AlreadyLocked);
        }
        match &mut new_swap {
            SwapInfo::Forward(swap_details) => {
                swap_details.nft_canister = nft_canister_id;
                swap_details.nft_id_string = nft_id_string;
                swap_details.status = SwapStatusForward::Init;
                swap_details.sale_id = "".to_string();
                swap_details.nft_id = nft_id;
                swap_details.created_at = time;
                swap_details.tokens_to_mint = tokens_to_mint;
                swap_details.escrow_sub_account = [0u8; 32];
                swap_details.gldt_receiver = Account {
                    owner: user_principal,
                    subaccount: None,
                };
            }
            _ => {}
        }

        if errors.len() > 0 {
            Err((new_swap, errors))
        } else {
            Ok(new_swap)
        }
    }

    pub fn forward() -> SwapBuilder<SwapDetailForward> {
        SwapBuilder::<SwapDetailForward>::new()
    }
}

impl SwapBuilder<SwapDetailReverse> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn _parse_nft_metadata_weight(&self, weight: String) -> Option<u16> {
        let mut weight = weight;
        weight.pop();
        match weight.parse::<u16>() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    fn _verify_nft_weight(
        &self,
        prin: &Principal
    ) -> Result<(Principal, NftCanisterConf, Option<FeeAccount>), NftValidationError> {
        let gldnft_canisters = read_state(|s| s.data.gldnft_canisters.clone());

        match gldnft_canisters.iter().find(|(principal, ..)| principal == prin) {
            Some(conf) => Ok(conf.clone()),
            None => Err(NftValidationError::InvalidNftWeight),
        }
    }

    pub async fn _get_origyn_id_string(
        &self,
        nft_id: &NftID,
        nft_canister_id: &Principal
    ) -> Result<String, NftValidationError> {
        match
            retry_async(|| get_nat_as_token_id_origyn(nft_canister_id.clone(), &nft_id.0), 3).await
        {
            Ok(id) => {
                return Ok(id);
            }
            Err((rejection_code, msg)) => {
                let msg = format!("{rejection_code:?}. {msg}");
                return Err(
                    NftValidationError::CantGetOrigynID(
                        format!("Can't get origyn ID for NFT nat : {nft_id:?}. error = {msg}")
                    )
                );
            }
        }
    }

    pub async fn _is_owned_by_canister(
        &self,
        nft_id: &NftID,
        nft_canister_id: &Principal
    ) -> Result<(), NftValidationError> {
        let this_canister_id = read_state(|s| s.env.canister_id());
        match
            retry_async(|| icrc7_owner_of(nft_canister_id.clone(), vec![nft_id.0.clone()]), 3).await
        {
            Ok(res) => {
                if let Some(Some(Account3 { owner, .. })) = res.get(0) {
                    if owner == &this_canister_id {
                        Ok(())
                    } else {
                        Err(NftValidationError::NotOwnedBySwapCanister)
                    }
                } else {
                    Err(NftValidationError::CantVerifySwapCanisterOwnsNft)
                }
            }
            Err(_) => Err(NftValidationError::CantVerifySwapCanisterOwnsNft),
        }
    }

    pub async fn init(
        self,
        init_args: &SwapTokensForNftArgs,
        user_principal: &Principal
    ) -> Result<SwapInfo, (SwapInfo, Vec<NftValidationError>)> {
        // we need to query the nft meta details to get the correct grams and serial maybe.
        let mut errors: Vec<NftValidationError> = vec![];
        let correct_weight = match self._verify_nft_weight(&init_args.nft_canister_id) {
            Ok((_, weight, _)) => weight.grams,
            Err(err) => {
                errors.push(err);
                0u16
            }
        };
        let tokens_to_receive = match GldtNumTokens::new_from_weight(correct_weight) {
            Ok(tokens) => tokens,
            Err(_) => {
                errors.push(NftValidationError::InvalidGldtTokensFromWeight);
                GldtNumTokens::invalid()
            }
        };

        let _ = self
            ._is_owned_by_canister(&init_args.nft_id, &init_args.nft_canister_id).await
            .map_err(|e| {
                errors.push(e);
            });

        let nft_id_string = match
            self._get_origyn_id_string(&init_args.nft_id, &init_args.nft_canister_id).await
        {
            Ok(id) => id,
            Err(e) => {
                errors.push(e);
                "".to_string()
            }
        };

        // create the new swap
        let mut new_swap = SwapInfo::new(SwapType::Reverse);
        let swap_status = if &errors.len() > &0 {
            SwapStatusReverse::Failed(SwapErrorReverse::NftValidationFailed(errors.clone()))
        } else {
            SwapStatusReverse::Init
        };
        match &mut new_swap {
            SwapInfo::Reverse(swap_details) => {
                swap_details.nft_id = init_args.nft_id.clone();
                swap_details.nft_id_string = nft_id_string;
                swap_details.nft_canister = init_args.nft_canister_id;
                swap_details.status = swap_status;
                swap_details.created_at = timestamp_millis();
                swap_details.tokens_to_receive = tokens_to_receive;
                swap_details.user = user_principal.clone();
            }
            _ => {}
        }

        if errors.len() > 0 {
            Err((new_swap, errors))
        } else {
            Ok(new_swap)
        }
    }

    pub fn reverse() -> SwapBuilder<SwapDetailReverse> {
        SwapBuilder::<SwapDetailReverse>::new()
    }
}
