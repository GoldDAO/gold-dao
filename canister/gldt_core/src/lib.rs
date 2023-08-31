//! GLDT is a digital token 100% backed by physical gold in the form
//! of NFTs in a ratio of 1 gram of gold NFTs equals 100 GLDT. The
//! NFTs have their ownership registered to this canister, which is
//! used to convert NFTs to GLDT and back.  The GLDT canister
//! purchases NFTs by minting tokens and sells NFTs against the
//! burning of tokens.
//!
//! The code of this canister is generic in the sense that it is not
//! tied to any particular type of NFT, except for the notion of
//! 'grams' which is tied to tokens in a ratio of one gram equals 100
//! tokens. Thus, in principle, the same code can be used for NFT of
//! any physical commodity measured in grams. The cansiter could
//! generalized further by replacing grams by some generic quantity,
//! but doing so right now (2023) seems to have little benefit and only
//! hamper the readability of the code.
//!
//! The GLDT canister collaborates with the canisters holding gold
//! NFTs as well as the GLDT ledger, which an instance of a standard
//! IC ledger.
//!
//!
//! ```
//! NFT                  GLDT            Ledger
//!  |                    |                |
//!  | offer request (1)  |                |
//!  +------------------->|                |
//!  |                    |   mint (2)     |
//!  |                    +--------------->|
//!  |                    |                |
//!  |                    |<---------------+
//!  |                    |                |
//!  |<-------------------+                +---+
//!  |        offer       |                |   | transact (3b)
//!  |                    |                |<--+
//!  +---+                |                |
//!  |   | accept (3a)    |                |
//!  |<--+                |                |
//!  |                    |                |
//! ```
//!
//! The lifecycle of one NFT is as follows.
//!
//! * Offer request made (1), i.e., an NFT canister requests an offer for
//! a particular NFT.
//!
//! * An offer is made by  minting (2) tokens to an escrow account of NFT
//! canister.
//!
//! * The offer is accepted (3a) on the NFT canister: the NFT now belongs
//! to GLDT canister and the minted tokens are in
//! circulation (3b). Alternatively, the offer is rejected and the tokens
//! are burned.
//!
//! * The view of the ownership of NFT from the NFT canister and from
//! the GLDT canister is periodically audited (to be implemented).
//!
//! * The GLDT canister releases an NFT against proof that the
//! corresponding number of tokens have been burned (to be
//! implemented).
//!
//! The GLDT ledger uses the account ID of the gldt cansiter (an
//! instance of this code) as its 'minting account'.  Computed as
//! `$(dfx ledger account-id --of-canister gldt)`. The GLDT canister
//! also needs to point to the ledger canister as given by `$(dfx
//! canister id ledger)`.

use candid::{ candid_method, CandidType, Deserialize, Principal, Nat };
// use canistergeek_ic_rust::logger::log_message;
use ic_cdk::{ api, api::call::CallResult, call, storage };
use ic_cdk_macros::{ init, query, update };
// use ic_ledger_types::Block;
// use ic_ledger_types::{AccountIdentifier, Memo, Subaccount, Tokens};
// use ic_ledger_types::ic;
use icrc_ledger_types::icrc1::{
    account::{ Account, Subaccount },
    transfer::{ BlockIndex, Memo, NumTokens, TransferArg, TransferError },
};
use serde::Serialize;
// use serde_bytes::ByteBuf;
// use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::hash::Hash;

// mod declarations;
mod declarations;
use declarations::gld_nft_manual_2::{
    self,
    ManageSaleRequest,
    BidRequest,
    SaleStatusShared,
    Account as OrigynAccount,
    SubAccountInfo,
    EscrowReceipt,
    TokenSpec,
    ICTokenSpec,
    SaleStatusShared_sale_type,
    PricingConfigShared__1,
    AskFeature,
    ManageSaleResult,
    EscrowRequest,
    DepositDetail,
    SaleInfoRequest,
    SaleInfoResult,
    SaleInfoResponse,
    SubAccountInfo_account,
};

/// Constants
pub const GLDT_SUBDIVIDABLE_BY: u64 = 100_000_000;
pub const GLD_NFT_ESCROW_FEE_ADDITION: u64 = 10_000 * 10;

/// The configuration points to the canisters that this canister
/// collaborates with, viz., the GLDT ledger canister and the NFT
/// canisters.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    gld_nft_canister_ids: Vec<(Principal, NftCanisterConf)>,
}

/// Configuration information for a single NFT canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct NftCanisterConf {
    /// The size in grams of the physical NFTs managed by this
    /// canister.  Note that the max value of u16 in grams is over
    /// 65kg. The largest gold bars are 400oz (~11kg) and the largest
    /// silver bars are 1000oz (~31kg).
    grams: u16,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_ids: Vec::new(),
        }
    }
}

/// An NFT is identified by a string.
type NftId = String;

/// Record of information about an NFT for which GLDT has been burned.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtBurned {
    /// The block height at which the tokens minted for this NFT were
    /// burned.
    burn_block_height: u64,
}

/// Record of information about an NFT for which GLDT has been minted.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtMinted {
    /// Block height when the GLDT was minted. Must be non-zero and
    /// point to a block with a minting transaction with the right
    /// number of tokens and subaccount.
    mint_block_height: BlockIndex,

    /// The last timestamp when this NFT was audited, i.e., when it
    /// was verified that this NFT belongs to this canister, or zero
    /// if no audit has been made.
    last_audited_timestamp_seconds: u64,

    /// The end of an NFT lifecycle in the GLDT canister is the
    /// burning of the minted tokens and the release of the
    /// corresponding NFT.
    burned: Option<GldtBurned>,
}

/// Record of information about an NFT for which an offer has been made.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldNft {
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gld_nft_canister_id: Principal,
    /// The number of grams that this NFT is reported to have.
    grams: u16,
    /// This field is passed verbatim from the offer request.
    requested_memo: Memo,
    /// The subaccount to which tokens are minted. This is always a subaccount of a GLD NFT canister.
    to_subaccount: Subaccount,
    /// The account who owned the NFT and triggered the swap for GLDT.
    receiving_account: Account,
    /// The timestamp when the request to issue GLDT was issued.
    gldt_minting_timestamp_seconds: u64,

    /// Filled when tokens are successfully minted.
    minted: Option<GldtMinted>,

    /// Optional reference to a previous minting/burning pair for this
    /// NFT as a historial record. If specified, the record must
    /// satisfy 'is_burned'.
    older_record: Option<Box<GldNft>>,
}

/// Record of successful minting or burning of GLDT for GLD NFTs
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtRecord {
    /// The index of the transaction record
    index: BlockIndex,
    /// The type of transaction, either "mint" or "burn"
    record_type: String,
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gld_nft_canister_id: Principal,
    /// The number of grams that this NFT is reported to have.
    grams: u16,
    /// The id of the NFT that was locked up
    nft_id: NftId,
    /// The account who owned the NFT and triggered the swap for GLDT.
    receiving_account: Account,
    /// The timestamp when the request to issue GLDT was issued.
    gldt_minting_timestamp_seconds: u64,
    /// The amount of tokens minted.
    gldt_minted: NumTokens,
    /// The block index when the GLDT were minted or burned.
    block_height: BlockIndex,
}

impl GldNft {
    fn is_burned(&self) -> bool {
        if let Some(minted) = &self.minted {
            if let Some(burned) = &minted.burned { burned.burn_block_height > 0 } else { false }
        } else {
            false
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct GldtService {
    conf: Conf,
    nfts: BTreeMap<(Principal, NftId), GldNft>,
    records: BTreeMap<BlockIndex, GldtRecord>,
}

thread_local! {
    /* stable */
    static SERVICE: RefCell<GldtService> = RefCell::default();
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("executing pre_upgrade");
    canistergeek_ic_rust::logger::log_message(format!("executing pre_upgrade"));

    // canister geek data
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    let service = SERVICE.with(|cell| cell.borrow_mut().clone());

    storage::stable_save((service, monitor_stable_data, logger_stable_data)).unwrap();
    // SERVICE.with(|cell| storage::stable_save((cell.take(),)).unwrap());
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    // let (service, monitor_stable_data, logger_stable_data) = storage::stable_restore().unwrap();

    // SERVICE.with(|cell| *cell.borrow_mut() = service);
    // canister geek data
    let stable_data: Result<
        (
            GldtService,
            canistergeek_ic_rust::monitor::PostUpgradeStableData,
            canistergeek_ic_rust::logger::PostUpgradeStableData,
        ),
        String
    > = storage::stable_restore();
    match stable_data {
        Ok((service, monitor_stable_data, logger_stable_data)) => {
            SERVICE.with(|cell| {
                *cell.borrow_mut() = service;
            });
            canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
            canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
        }
        Err(_) => {}
    }

    ic_cdk::println!("executed post_upgrade");
    canistergeek_ic_rust::logger::log_message(format!("executed post_upgrade"));
}

#[init]
#[candid_method(init)]
fn init(conf: Option<Conf>) {
    if let Some(conf) = conf {
        canistergeek_ic_rust::logger::log_message(
            format!(
                "INFO :: new config: gldt_ledger_canister_id = {}, gld_nft_canister_ids = {:?}",
                conf.gldt_ledger_canister_id,
                conf.gld_nft_canister_ids
            )
        );
        SERVICE.with(|s| {
            s.borrow_mut().conf = conf;
        })
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GetRecordsRequest {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GetRecordsResponse {
    total: u64,
    data: Option<Vec<GldtRecord>>,
}

#[query]
#[candid_method(query)]
fn get_records(req: GetRecordsRequest) -> GetRecordsResponse {
    let page = match req.page {
        Some(val) => {
            if val < 1 { 1 } else { val }
        }
        None => 1,
    };
    let limit = match req.limit {
        Some(val) => if val < 1 { 10 } else if val > 100 { 100 } else { val }
        None => 10,
    };
    let res: GetRecordsResponse = SERVICE.with(|s| {
        let records = &mut s.borrow_mut().records;
        let start = (page - 1) * limit;
        let paginated_records = records.values().skip(start).take(limit).cloned().collect();
        return GetRecordsResponse { total: records.len() as u64, data: Some(paginated_records) };
    });
    return res;
}

#[update]
#[candid_method(update)]
fn get_conf() -> Conf {
    canistergeek_ic_rust::logger::log_message(format!("INFO :: get_conf"));
    SERVICE.with(|s| s.borrow_mut().conf.clone())
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct OfferRequest {
    nft_id: NftId,
    to_subaccount: Subaccount,
    receiving_account: Account,
    requested_memo: Memo,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Offer {
    tokens_minted: NumTokens,
    block_height: BlockIndex,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct InfoRequest {
    source_canister: Principal,
    nft_id: NftId,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct NftInfo {
    info: Option<GldNft>,
}

type TransferResult = Result<BlockIndex, TransferError>;
// #[derive(CandidType, Deserialize, Clone, Debug)]
// pub enum TransferResult {
//     Ok(BlockIndex),
//     Err(TransferError),
// }

#[update]
#[candid_method(update)]
async fn nft_info(args: InfoRequest) -> NftInfo {
    canistergeek_ic_rust::logger::log_message(format!("INFO :: nft_info. Arguments: {:?}", args));
    SERVICE.with(|s| NftInfo {
        info: s
            .borrow()
            .nfts.get(&(args.source_canister, args.nft_id))
            .map(|x| x.clone()),
    })
}

async fn icrc1_transfer(arg: TransferArg) -> CallResult<(TransferResult,)> {
    let gldt_ledger_canister_id = SERVICE.with(|s| -> Principal {
        s.borrow().conf.gldt_ledger_canister_id
    });
    call(gldt_ledger_canister_id, "icrc1_transfer", (arg,)).await
}

fn delete_nft_entry_from_list(nft_id: &NftId) -> Result<(), String> {
    let the_caller = api::caller();
    SERVICE.with(|s| {
        let nfts = &mut s.borrow_mut().nfts;
        match nfts.entry((the_caller, nft_id.to_string())) {
            btree_map::Entry::Occupied(o) => {
                o.remove_entry();
                Ok(())
            }
            _ => Err(format!("NFT ID {} not found in list.", nft_id)),
        }
    })
}

#[update]
#[candid_method(update)]
async fn request_offer(args: OfferRequest, bid: BidRequest) -> Result<Offer, String> {
    canistergeek_ic_rust::logger::log_message(format!("INFO :: offer request {:?}", args.clone()));
    if args.nft_id.clone().is_empty() {
        return Err(String::from("NFT ID cannot be empty"));
    }

    // Find the caller.
    let the_caller = api::caller();
    // Extract configuration and validate caller.
    let (gld_nft_canister_id, gld_nft_conf) = SERVICE.with(
        |s| -> Result<(Principal, NftCanisterConf), String> {
            let conf = &s.borrow().conf;
            let (gld_nft_canister_id, gld_nft_conf) = conf.gld_nft_canister_ids
                .iter()
                .find(|(x, _)| *x == the_caller)
                .ok_or_else(|| {
                    format!(
                        "invalid caller: was {}, expected one of {:?}",
                        the_caller,
                        conf.gld_nft_canister_ids
                            .iter()
                            .map(|(x, _)| x)
                            .collect::<Vec<_>>()
                    )
                })?;

            Ok((gld_nft_canister_id.clone(), gld_nft_conf.clone()))
        }
    )?;

    // Time is in nanos.
    let timestamp_seconds = api::time() / 1_000_000_000;
    // 100 tokens per gram.
    let tokens_minted: NumTokens = NumTokens::from(
        GLDT_SUBDIVIDABLE_BY * (gld_nft_conf.grams as u64) * 100
    );
    // Insert a new entry for this nft_id. If an entry is already
    // present, replace it if inactive; otherwise it is an error to
    // request a new offer for this NFT.
    SERVICE.with(|s| {
        let nfts = &mut s.borrow_mut().nfts;
        let new_entry = GldNft {
            gld_nft_canister_id: gld_nft_canister_id.clone(),
            grams: gld_nft_conf.grams.clone(),
            requested_memo: args.requested_memo.clone(),
            to_subaccount: args.to_subaccount.clone(),
            receiving_account: args.receiving_account.clone(),
            gldt_minting_timestamp_seconds: timestamp_seconds.clone(),
            minted: None,
            older_record: None,
        };
        match nfts.entry((the_caller, args.nft_id.clone())) {
            btree_map::Entry::Vacant(v) => {
                v.insert(new_entry);
            }
            btree_map::Entry::Occupied(mut o) => {
                if o.get().is_burned() {
                    canistergeek_ic_rust::logger::log_message(
                        format!(
                            "INFO :: replacing inactive entry for NFT: {}: old entry: {:?}",
                            args.nft_id.clone(),
                            o.get()
                        )
                    );
                    o.insert(GldNft {
                        older_record: Some(Box::new(o.get().clone())),
                        ..new_entry
                    });
                } else {
                    let msg = format!(
                        "There is already an active entry for NFT: {}. Replacing it to move on. REMOVE THIS BEFORE PRODUCTION.",
                        args.nft_id.clone()
                    );
                    canistergeek_ic_rust::logger::log_message(msg);

                    o.insert(GldNft {
                        older_record: Some(Box::new(o.get().clone())),
                        ..new_entry
                    });
                    // let msg = format!(
                    //     "There is already an active entry for NFT: {}. Canceling new minting of tokens.",
                    //     args.nft_id.clone()
                    // );
                    // return Err(msg);
                }
            }
        }
        // Ok(())
    });

    let transfer_args = TransferArg {
        memo: Some(args.requested_memo.clone()),
        amount: tokens_minted.clone(),
        fee: None,
        from_subaccount: None,
        to: Account {
            owner: the_caller,
            // owner: api::id(),
            subaccount: Some(args.to_subaccount.clone()),
        },
        created_at_time: None,
    };

    let result: TransferResult = match icrc1_transfer(transfer_args.clone()).await {
        Ok((v,)) => v,
        Err((code, message)) => {
            let _ = delete_nft_entry_from_list(&args.nft_id);
            return Err(
                format!("Error while calling icrc1_transfer. Code {:?}, Message: {}", code, message)
            );
        }
    };
    let block_height: BlockIndex = match result.clone() {
        Ok(height) => height,
        Err(e) => {
            let _ = delete_nft_entry_from_list(&args.nft_id);
            return Err(
                format!(
                    "Error while executing icrc1_transfer with args {:?}. Message: {:?}",
                    transfer_args,
                    e
                )
            );
        }
    };
    canistergeek_ic_rust::logger::log_message(
        format!(
            "INFO :: minted {} GLDT at block {} to prinicpal {} with subaccount {:?}",
            tokens_minted.clone(),
            block_height.clone(),
            transfer_args.to.owner,
            transfer_args.to.subaccount
        )
    );

    SERVICE.with(|s| {
        let records = &mut s.borrow_mut().records;
        let new_index: BlockIndex = match records.last_key_value() {
            Some((last_index, _)) => (*last_index).clone() + Nat::from(1),
            None => Nat::from(0),
        };

        let new_record = GldtRecord {
            index: new_index.clone(),
            record_type: "mint".to_string(),
            gld_nft_canister_id: gld_nft_canister_id.clone(),
            grams: gld_nft_conf.grams.clone(),
            nft_id: args.nft_id.clone(),
            receiving_account: args.receiving_account.clone(),
            gldt_minting_timestamp_seconds: timestamp_seconds.clone(),
            gldt_minted: tokens_minted.clone(),
            block_height: block_height.clone(),
        };
        records.insert(new_index, new_record)
    });

    // Accept the NFT sale
    let _res = accept_offer(bid).await?;

    // let accept_args: ManageSaleRequest = ManageSaleRequest::bid(bid_request);
    // let nft_result: NftTransferResult = call(gld_nft_canister_id, "sale_nft_origyn", accept_args);
    // service.sale_nft_origyn(accept_args);

    // IMPORTANT: don't panic after async call!!!
    //
    // The code that follows looks rather complicated, but it is
    // almost only error checking due to the fact that we cannot be
    // sure that the canister state before and after the async call
    // match, even for the entry of the NFT being offered.
    SERVICE.with(|s| {
        // IMPORTANT: log instead of trap!
        let nfts = &mut s.borrow_mut().nfts;
        match nfts.entry((the_caller, args.nft_id.clone())) {
            btree_map::Entry::Vacant(v) => {
                // This should never happen as an entry for this NFT
                // was created before the async call was made, but we
                // try to handle it gracefully anyway. Perhaps the
                // canister was upgraded in between and state was lost.
                ic_cdk::println!(
                    "WARNING: tokens minted, but no entry to update! Attempting to rectify by creating a new entry. Request: {:?}, Transfer {:?}, block height: {}",
                    args,
                    transfer_args,
                    block_height
                );
                v.insert(GldNft {
                    gld_nft_canister_id: the_caller,
                    grams: gld_nft_conf.grams,
                    requested_memo: args.requested_memo,
                    to_subaccount: args.to_subaccount,
                    receiving_account: args.receiving_account.clone(),
                    gldt_minting_timestamp_seconds: timestamp_seconds,
                    minted: Some(GldtMinted {
                        mint_block_height: block_height.clone(),
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                    }),
                    older_record: None,
                });
            }
            btree_map::Entry::Occupied(mut o) => {
                // Do sanity checking.
                let mut problems = Vec::new();
                if o.get().gld_nft_canister_id != the_caller {
                    problems.push(
                        format!(
                            "NFT canister ID - recorded: {}, expected: {}",
                            o.get().gld_nft_canister_id,
                            the_caller
                        )
                    );
                }
                if o.get().grams != gld_nft_conf.grams {
                    problems.push(
                        format!(
                            "weight in grams - recorded: {}, expected: {}",
                            o.get().grams,
                            gld_nft_conf.grams
                        )
                    );
                }
                if o.get().requested_memo != args.requested_memo {
                    problems.push(
                        format!(
                            "memo - recorded: {:?}, expected: {:?}",
                            o.get().requested_memo,
                            args.requested_memo
                        )
                    );
                }
                if o.get().to_subaccount != args.to_subaccount {
                    problems.push(
                        format!(
                            "escrow subaccount - recorded: {:?}, expected: {:?}",
                            o.get().to_subaccount,
                            args.to_subaccount
                        )
                    );
                }
                if o.get().gldt_minting_timestamp_seconds != timestamp_seconds {
                    problems.push(
                        format!(
                            "timestamp - recorded: {}, expected: {}",
                            o.get().gldt_minting_timestamp_seconds,
                            timestamp_seconds
                        )
                    );
                }
                if !problems.is_empty() {
                    // If there are problems, it is most likely the
                    // case that the response we are handing is
                    // spurious, i.e., not corresponding to the
                    // request made.
                    let msg = format!(
                        "ERROR: ignoring canister response because request state does not match response state: problems {}, response {:?}",
                        problems.join("; "),
                        result
                    );
                    canistergeek_ic_rust::logger::log_message(msg.clone());
                    ic_cdk::println!("{}", msg);
                } else {
                    match &o.get().minted {
                        None => {
                            // This is the happy path.
                            ic_cdk::println!(
                                "INFO: offer for NFT {} with block height {}",
                                args.nft_id,
                                block_height
                            );
                            o.get_mut().minted = Some(GldtMinted {
                                mint_block_height: block_height.clone(),
                                last_audited_timestamp_seconds: 0,
                                burned: None,
                            });
                        }
                        // This should never happen as a request to mint
                        // token is only issued with the 'minted' set to
                        // None. But we try to handle it as gracefully as
                        // possible.
                        Some(minted) => {
                            if minted.burned.is_some() {
                                ic_cdk::println!(
                                    "WARNING: offer for NFT {} with block height {} - inactive entry overwritten {:?}",
                                    args.nft_id,
                                    block_height,
                                    minted
                                );
                                o.get_mut().minted = Some(GldtMinted {
                                    mint_block_height: block_height.clone(),
                                    last_audited_timestamp_seconds: 0,
                                    burned: None,
                                });
                            } else {
                                // If the block heights are equal, there is no issue
                                if minted.mint_block_height != block_height.clone() {
                                    ic_cdk::println!(
                                        "ERROR: possible double minting for NFT {}; tokens minted to {:?} at block height {}, previous record indicating minting at block height {}",
                                        args.nft_id,
                                        args.to_subaccount,
                                        block_height.clone(),
                                        minted.mint_block_height
                                    );
                                } else {
                                    ic_cdk::println!(
                                        "WARNING: ignoring double response for NFT {}",
                                        args.nft_id
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    // Final result only depends on whether or not minting was successful.
    Ok(Offer {
        tokens_minted,
        block_height,
    })
}

async fn get_escrow_info(escrow: EscrowRequest) {
    let service = gld_nft_manual_2::SERVICE(api::caller());
    canistergeek_ic_rust::logger::log_message(
        format!("Sending get_escrow_info to canister {}", api::caller())
    );
    match service.sale_nft_origyn(ManageSaleRequest::escrow_deposit(escrow)).await {
        Ok((res,)) => {
            match res {
                ManageSaleResult::ok(val) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("get_escrow_info :: Successfuly response: {:?}", val)
                    );
                    // return Ok(());
                }
                ManageSaleResult::err(err) => canistergeek_ic_rust::logger::log_message(err.text),
            }
        }
        Err((_, msg)) =>
            canistergeek_ic_rust::logger::log_message(
                format!("get_escrow_info :: Severe error while accepting offer. Message: {}", msg)
            ),
    }
    // return ();
}

async fn accept_offer(bid: BidRequest) -> Result<(), String> {
    let service = gld_nft_manual_2::SERVICE(api::caller());
    canistergeek_ic_rust::logger::log_message(format!("Placing bid with arguments {:?}", bid));
    let res = match service.sale_nft_origyn(ManageSaleRequest::bid(bid)).await {
        Ok((res,)) => {
            match res {
                ManageSaleResult::ok(val) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Successfuly response: {:?}", val)
                    );
                    return Ok(());
                }
                ManageSaleResult::err(err) => Err(err.text),
            }
        }
        Err((_, msg)) => Err(format!("Severe error while accepting offer. Message: {}", msg)),
    };

    return res;
}

// async fn mint_to_address() {}

async fn get_escrow_request(receipt: EscrowReceipt) -> Result<SubAccountInfo_account, ()> {
    let service = gld_nft_manual_2::SERVICE(api::caller());
    canistergeek_ic_rust::logger::log_message(
        format!("Sending sale_info_nft_origyn(escrow_info) to canister {}", api::caller())
    );
    let res = (match service.sale_info_nft_origyn(SaleInfoRequest::escrow_info(receipt)).await {
        Ok((res,)) => {
            match res {
                SaleInfoResult::ok(SaleInfoResponse::escrow_info(val)) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Response of sale_info_nft_origyn(escrow_info) : {:?}", val)
                    );
                    return Ok(val.account);
                }
                SaleInfoResult::err(msg) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Failed reponse from sale_info_nft_origyn : {:?}", msg)
                    );
                    Err(())
                }
                _ => Err(()),
            }
        }
        Err((_, msg)) => {
            canistergeek_ic_rust::logger::log_message(
                format!("sale_info_nft_origyn :: Severe error while accepting offer. Message: {}", msg)
            );
            Err(())
        }
    })?;
    return res;
}

async fn get_deposit_address() -> Result<SubAccountInfo_account, ()> {
    let service = gld_nft_manual_2::SERVICE(api::caller());
    canistergeek_ic_rust::logger::log_message(
        format!("Sending sale_info_nft_origyn(deposit_info) to canister {}", api::caller())
    );
    let res = (match
        service.sale_info_nft_origyn(
            SaleInfoRequest::deposit_info(Some(OrigynAccount::principal(api::id())))
        ).await
    {
        Ok((res,)) => {
            match res {
                SaleInfoResult::ok(SaleInfoResponse::deposit_info(val)) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Response of sale_info_nft_origyn(deposit_info) : {:?}", val)
                    );
                    return Ok(val.account);
                }
                SaleInfoResult::err(msg) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Failed reponse from sale_info_nft_origyn : {:?}", msg)
                    );
                    Err(())
                }
                _ => Err(()),
            }
        }
        Err((_, msg)) => {
            canistergeek_ic_rust::logger::log_message(
                format!("sale_info_nft_origyn :: Severe error while accepting offer. Message: {}", msg)
            );
            Err(())
        }
    })?;
    return res;
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriberNotification {
    escrow_info: SubAccountInfo,
    sale: SaleStatusShared,
    seller: OrigynAccount,
    collection: Principal,
}

#[update]
#[candid_method(update)]
async fn notify_sale_nft_origyn(args: SubscriberNotification) -> Result<String, String> {
    canistergeek_ic_rust::logger::log_message(format!("Sale notifcation: {:?}", args));
    canistergeek_ic_rust::monitor::collect_metrics();
    ic_cdk::println!("Sale notifcation: {:?}", args);
    // let subaccount: Subaccount = match args.escrow_info.account.sub_account.as_slice().try_into() {
    //     Ok(x) => x,
    //     Err(_) => {
    //         let msg = format!(
    //             "ERROR: expected a subaccount of length {} but it was {}",
    //             32,
    //             args.escrow_info.account.sub_account.len()
    //         );
    //         canistergeek_ic_rust::logger::log_message(msg.clone());
    //         return Err(msg);
    //     }
    // };
    let buyer = OrigynAccount::principal(api::id());
    let seller_icrc1: Account = (match args.seller.clone() {
        OrigynAccount::principal(p) =>
            Ok(Account {
                owner: p.clone(),
                subaccount: None,
            }),
        _ => {
            let msg = format!("No valid account found for seller.");
            canistergeek_ic_rust::logger::log_message(msg.clone());
            Err(msg)
        }
    })?;

    canistergeek_ic_rust::logger::log_message(
        format!(
            "owner from escrow_info of call message: {}",
            args.escrow_info.account.principal.to_text()
        )
    );

    // match buyer {
    //     OrigynAccount::account { owner, sub_account } =>
    // canistergeek_ic_rust::logger::log_message(
    //     format!("owner: {}, sub_account: {:?}", owner.to_text(), sub_account)
    // );
    //     _ => (),
    // }

    let (token, config) = match args.sale.sale_type {
        SaleStatusShared_sale_type::auction(t) => (t.token, t.config),
    };
    let amount = (match config {
        PricingConfigShared__1::ask(Some(features)) => {
            let mut amount: Nat = Nat::from(0);
            for feature in features {
                if let AskFeature::buy_now(val) = feature {
                    amount = val;
                }
            }
            if amount > Nat::from(0) {
                Ok(amount)
            } else {
                Err(String::from("buy_now price needs to be >0"))
            }
        }
        _ => Err(String::from("Couldn't find buy_now price.")),
    })?;

    let nft_id = args.sale.token_id;
    let subaccount: Subaccount = get_deposit_address().await
        .unwrap()
        .sub_account.as_slice()
        .try_into()
        .unwrap();

    let receipt = EscrowReceipt {
        amount: amount.clone(),
        token: token.clone(),
        buyer: buyer.clone(),
        seller: args.seller.clone(),
        token_id: nft_id.clone(),
    };
    let tmp: Subaccount = get_escrow_request(receipt).await
        .unwrap()
        .sub_account.as_slice()
        .try_into()
        .unwrap();
    canistergeek_ic_rust::logger::log_message(format!("Subaccount from manual request: {:?}", tmp));

    let request = OfferRequest {
        nft_id: nft_id.clone(),
        // to_subaccount: subaccount,
        to_subaccount: tmp,
        receiving_account: seller_icrc1,
        requested_memo: Memo::from(0),
    };

    let escrow = EscrowRequest {
        token_id: nft_id.clone(),
        deposit: DepositDetail {
            token: token.clone(),
            amount: amount.clone(),
            buyer: buyer.clone(),
            seller: args.seller.clone(),
            sale_id: None,
            trx_id: None,
        },
        lock_to_date: None,
    };

    // get_escrow_info(escrow).await;

    let bid = BidRequest {
        broker_id: None,
        sale_id: args.sale.sale_id,
        escrow_receipt: EscrowReceipt {
            token,
            seller: args.seller,
            buyer,
            token_id: nft_id.clone(),
            amount,
        },
    };

    // validate_inputs()
    // mint_tokens()
    // accept_offer()
    // update_registry()
    // clean_up()

    match request_offer(request, bid).await {
        Ok(r) => {
            let msg = format!("INFO: minted {} GLDT at block {}", r.tokens_minted, r.block_height);
            canistergeek_ic_rust::logger::log_message(msg.clone());
            return Ok(msg);
        }
        Err(e) => {
            let msg = format!("ERROR: {}", e);
            canistergeek_ic_rust::logger::log_message(msg.clone());
            return Err(msg);
        }
    }
}

// for monitoring during development
#[query(name = "getCanistergeekInformation")]
#[candid_method(query)]
async fn get_canistergeek_information(
    request: canistergeek_ic_rust::api_type::GetInformationRequest
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    validate_caller();
    canistergeek_ic_rust::get_information(request)
}

#[update(name = "updateCanistergeekInformation")]
#[candid_method(query)]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest
) -> () {
    validate_caller();
    canistergeek_ic_rust::update_information(request);
}

fn validate_caller() -> () {}

#[cfg(not(test))]
candid::export_service!();

#[cfg(not(test))]
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[test]
fn check_candid_interface() {
    use candid::utils::{ service_compatible, CandidSource };
    use std::path::Path;

    candid::export_service!();
    let new_interface = __export_service();

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("src/gldt_core.did"))
    ).unwrap();
}

// export_candid!();
