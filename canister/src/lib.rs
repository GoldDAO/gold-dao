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

use candid::{candid_method, CandidType, Principal};
use ic_cdk::{api, storage};
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, Memo, Subaccount, Tokens};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::hash::Hash;

/// The configuration points to the canisters that this canister
/// collaborates with, viz., the GLDT ledger canister and the NFT
/// canisters.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    gldt_nft_canister_ids: Vec<(Principal, NftCanisterConf)>,
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
            gldt_nft_canister_ids: Vec::new(),
        }
    }
}

/// An NFT is identified by a string.
type NftId = String;

/// Record of information about an NFT for which GLDT has been burned.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtNftBurned {
    /// The block height at which the tokens minted for this NFT were
    /// burned.
    burn_block_height: u64,
}

/// Record of information about an NFT for which GLDT has been minted.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtNftMinted {
    /// Block height when the GLDT was minted. Must be non-zero and
    /// point to a block with a minting transaction with the right
    /// number of tokens and subaccount.
    mint_block_height: u64,

    /// The last timestamp when this NFT was audited, i.e., when it
    /// was verified that this NFT belongs to this canister, or zero
    /// if no audit has been made.
    last_audited_timestamp_seconds: u64,

    /// The end of an NFT lifecycle in the GLDT canister is the
    /// burning of the minted tokens and the release of the
    /// corresponding NFT.
    burned: Option<GldtNftBurned>,
}

/// Record of information about an NFT for which an offer has been made.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtNft {
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gldt_nft_canister_id: Principal,
    /// The number of grams that this NFT is reported to have.
    grams: u16,
    /// This field is passed verbatim from the offer request.
    requested_memo: Memo,
    /// The subaccount to which tokens are minted.
    to_subaccount: Subaccount,
    /// The timestamp when the request to issue GLDT was issued.
    gldt_minting_timestamp_seconds: u64,

    /// Filled when tokens are successfully minted.
    minted: Option<GldtNftMinted>,

    /// Optional reference to a previous minting/burning pair for this
    /// NFT as a historial record. If specified, the record must
    /// satisfy 'is_burned'.
    older_record: Option<Box<GldtNft>>,
}

impl GldtNft {
    fn is_burned(&self) -> bool {
        if let Some(minted) = &self.minted {
            if let Some(burned) = &minted.burned {
                burned.burn_block_height > 0
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct GldtService {
    conf: Conf,
    nfts: BTreeMap<(Principal, NftId), GldtNft>,
}

thread_local! {
    /* stable */
    static SERVICE: RefCell<GldtService> = RefCell::default();
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("executing pre_upgrade");
    SERVICE.with(|cell| storage::stable_save((cell.take(),)).unwrap());
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("executing post_upgrade");
    let (service,): (GldtService,) = storage::stable_restore().unwrap();
    SERVICE.with(|cell| *cell.borrow_mut() = service);
}

#[init]
#[candid_method(init)]
fn init(conf: Option<Conf>) {
    if let Some(conf) = conf {
        ic_cdk::println!(
            "INFO: new config: gldt_ledger_canister_id = {}, gldt_nft_canister_ids = {:?}",
            conf.gldt_ledger_canister_id,
            conf.gldt_nft_canister_ids
        );
        SERVICE.with(|s| s.borrow_mut().conf = conf)
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct OfferRequest {
    nft_id: NftId,
    to_subaccount: Subaccount,
    requested_memo: Memo,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Offer {
    tokens_minted: Tokens,
    block_height: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct InfoRequest {
    source_canister: Principal,
    nft_id: NftId,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct NftInfo {
    info: Option<GldtNft>,
}

#[query]
#[candid_method(query)]
fn nft_info(args: InfoRequest) -> NftInfo {
    SERVICE.with(|s| NftInfo {
        info: s
            .borrow()
            .nfts
            .get(&(args.source_canister, args.nft_id))
            .map(|x| x.clone()),
    })
}

#[update]
#[candid_method(update)]
async fn request_offer(args: OfferRequest) -> Result<Offer, String> {
    ic_cdk::println!("INFO: offer request {:?}", args);
    if args.nft_id.is_empty() {
        api::trap(&"NFT ID cannot be empty");
    }
    // Find the caller.
    let the_caller = api::caller();
    // Extract configuration and validate caller.
    let (conf, gldt_ledger_canister_id) = SERVICE.with(|s| {
        let conf = &s.borrow().conf;
        let nft_conf = match conf
            .gldt_nft_canister_ids
            .iter()
            .find(|(x, _)| *x == the_caller)
        {
            Some((_, c)) => c,
            None => api::trap(&format!(
                "invalid caller: was {}, expected one of {:?}",
                the_caller,
                conf.gldt_nft_canister_ids
                    .iter()
                    .map(|(x, _)| x)
                    .collect::<Vec<_>>()
            )),
        };
        (nft_conf.clone(), conf.gldt_ledger_canister_id)
    });
    // Time is in nanos.
    let timestamp_seconds = api::time() / 1_000_000_000;
    // 100 tokens per gram.
    let tokens_minted = Tokens::from_e8s(Tokens::SUBDIVIDABLE_BY * (conf.grams as u64) * 100);
    // Insert a new entry for this nft_id. If an entry is already
    // present, replace it if inactive; otherwise it is an error to
    // request a new offer for this NFT.
    SERVICE.with(|s| {
        let nfts = &mut s.borrow_mut().nfts;
        let new_entry = GldtNft {
            gldt_nft_canister_id: the_caller,
            grams: conf.grams,
            requested_memo: args.requested_memo,
            to_subaccount: args.to_subaccount,
            gldt_minting_timestamp_seconds: timestamp_seconds,
            minted: None,
            older_record: None,
        };
        match nfts.entry((the_caller, args.nft_id.clone())) {
            btree_map::Entry::Vacant(v) => {
                v.insert(new_entry);
            }
            btree_map::Entry::Occupied(mut o) => {
                if o.get().is_burned() {
                    ic_cdk::println!(
                        "INFO: replacing inactive entry for NFT: {}: old entry: {:?}",
                        args.nft_id,
                        o.get()
                    );
                    o.insert(GldtNft {
                        older_record: Some(Box::new(o.get().clone())),
                        ..new_entry
                    });
                } else {
                    api::trap(&format!(
                        "there is already an active entry for NFT: {}",
                        args.nft_id
                    ));
                }
            }
        }
    });
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: args.requested_memo,
        amount: tokens_minted,
        fee: Tokens::ZERO,
        from_subaccount: None,
        to: AccountIdentifier::new(&the_caller, &args.to_subaccount),
        created_at_time: None,
    };
    let result =
        match ic_ledger_types::transfer(gldt_ledger_canister_id, transfer_args.clone()).await {
            Ok(Ok(block_height)) => Ok(block_height),
            Ok(Err(e)) => Err(e.to_string()),
            Err((c, e)) => Err(format!("code: {:?}, message: {}", c, e)),
        };
    //
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
		match result.clone() {
		    Ok(block_height) => {
			ic_cdk::println!("WARNING: tokens minted, but no entry to update! Attempting to rectify by creating a new entry. Request: {:?}, Transfer {:?}, block height: {}", args, transfer_args, block_height);
			v.insert(GldtNft {
			    gldt_nft_canister_id: the_caller,
			    grams: conf.grams,
			    requested_memo: args.requested_memo,
			    to_subaccount: args.to_subaccount,
			    gldt_minting_timestamp_seconds: timestamp_seconds,
			    minted: Some(GldtNftMinted {
				mint_block_height: block_height,
				last_audited_timestamp_seconds: 0,
				burned: None,
			    }),
			    older_record: None,
			});
		    },
		    Err(e) => { ic_cdk::println!("WARNING: token minting failed, but no entry to update. Request: {:?}, Transfer {:?}, Error: {}", args, transfer_args, e); },
		}
	    },
	    btree_map::Entry::Occupied(mut o) => {
		// Do sanity checking.
		let mut problems = Vec::new();
		if o.get().gldt_nft_canister_id != the_caller {
		    problems.push(format!("NFT canister ID - recorded: {}, expected: {}", o.get().gldt_nft_canister_id, the_caller));
		}
		if o.get().grams != conf.grams {
		    problems.push(format!("weight in grams - recorded: {}, expected: {}", o.get().grams, conf.grams));
		}
		if o.get().requested_memo != args.requested_memo {
		    problems.push(format!("memo - recorded: {:?}, expected: {:?}", o.get().requested_memo, args.requested_memo));
		}
		if o.get().to_subaccount != args.to_subaccount {
		    problems.push(format!("escrow subaccount - recorded: {:?}, expected: {:?}", o.get().to_subaccount, args.to_subaccount));
		}
		if o.get().gldt_minting_timestamp_seconds != timestamp_seconds {
		    problems.push(format!("timestamp - recorded: {}, expected: {}", o.get().gldt_minting_timestamp_seconds, timestamp_seconds));
		}
		if !problems.is_empty() {
		    // If there are problems, it is most likely the
		    // case that the response we are handing is
		    // spurious, i.e., not corresponding to the
		    // request made.
		    ic_cdk::println!("ERROR: ignoring canister response because request state does not match response state: problems {}, response {:?}", problems.join("; "), result);
		} else {
		    match &o.get().minted {
			None => {
			    // This is the happy path.
			    match result.clone() {
				Ok(block_height) => {
				    ic_cdk::println!("INFO: offer for NFT {} with block height {}", args.nft_id, block_height);
				    o.get_mut().minted = Some(GldtNftMinted {
					mint_block_height: block_height,
					last_audited_timestamp_seconds: 0,
					burned: None,
				    });
				},
				Err(e) => {
				    ic_cdk::println!("WARNING: token minting failed for NFT {} with error {}", args.nft_id, e);
				    // We remove the entry so a new attempt can be made to mint tokens for the same NFT.
				    o.remove_entry();
				},
			    }
			},
			// This should never happen as a request to mint
			// token is only issued with the 'minted' set to
			// None. But we try to handle it as gracefully as
			// possible.
			Some(minted) => {
			    match result.clone() {
				Ok(block_height) => {
				    if minted.burned.is_some() {
					ic_cdk::println!("WARNING: offer for NFT {} with block height {} - inactive entry overwritten {:?}", args.nft_id, block_height, minted);
					o.get_mut().minted = Some(GldtNftMinted {
					    mint_block_height: block_height,
					    last_audited_timestamp_seconds: 0,
					    burned: None,
					});
				    } else {
					// If the block heights are equal, there is no issue
					if minted.mint_block_height != block_height {
					    ic_cdk::println!("ERROR: possible double minting for NFT {}; tokens minted to {:?} at block height {}, previous record indicating minting at block height {}", args.nft_id, args.to_subaccount, block_height, minted.mint_block_height);
					} else {
					    ic_cdk::println!("WARNING: ignoring double response for NFT {}", args.nft_id);
					}
				    }
				},
				Err(e) => {
				    ic_cdk::println!("WARNING: NFT {} already has minted tokens and a second minting failed: {}", args.nft_id, e);
				},
			    }
			},
		    }
		}
	    }
	}
    });
    // Final result only depends on whether or not minting was successful.
    result.map(|block_height| Offer {
        tokens_minted,
        block_height,
    })
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct SubscriberNotification {
    escrow_info: SubAccountInfo,
    sale: SaleStatusShared,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct SubAccoutInfo2 {
    sub_account: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct SubAccountInfo {
    account: SubAccoutInfo2,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct SaleStatusShared {
    token_id: String,
}

#[update]
#[candid_method(update)]
async fn notify_sale_nft_origyn(args: SubscriberNotification) -> () {
    let subaccount: [u8; 32] = args
        .escrow_info
        .account
        .sub_account
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            api::trap(&format!(
                "ERROR: expected a subaccount of length {} but it was {}",
                32,
                v.len()
            ))
        });
    let request = OfferRequest {
        nft_id: args.sale.token_id,
        to_subaccount: Subaccount(subaccount),
        requested_memo: ic_ledger_types::Memo(0),
    };
    match request_offer(request).await {
        Ok(_) => (),
        Err(e) => ic_cdk::println!("ERROR: ignoring error: {}", e),
    }
}

#[cfg(not(test))]
candid::export_service!();

#[cfg(not(test))]
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[test]
fn check_candid_interface() {
    use candid::utils::{service_compatible, CandidSource};
    use std::path::Path;

    candid::export_service!();
    let new_interface = __export_service();

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("src/gldt.did")),
    )
    .unwrap();
}

/*
// Helpful commands

// Getting the ledger canister code and interface.
% export IC_VERSION=b3b00ba59c366384e3e0cd53a69457e9053ec987 // 2023-05-22
% curl -o ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ledger-canister_notify-method.wasm.gz"
% gunzip ledger.wasm.gz
% curl -o ledger.private.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/ledger.did"

// With gldt and ledger in your dfx.
dfx start --clean

dfx canister create --all

dfx build gldt

dfx canister install gldt --argument '(opt record {gldt_ledger_canister_id=principal "'$(dfx canister id ledger)'"; gldt_nft_canister_ids=vec{record { principal "'$(dfx identity get-principal)'"; record { grams=10}}}})' --mode=reinstall

// Copy ledger.wasm to the right place first...
dfx deploy ledger --argument '(record{minting_account="'$(dfx ledger account-id --of-canister gldt)'"; send_whitelist=vec{}; initial_values=vec{}})' --mode=reinstall 

// Try it out
dfx canister call gldt request_offer '(record {nft_id = "test2"; requested_memo=(2:nat64); to_subaccount = blob "abcdefghijklmnopqrstuvxyz1234567"})'

dfx canister call gldt nft_info '(record {source_canister = principal "'$(dfx identity get-principal)'"; nft_id = "test2";})'

dfx canister call gldt notify_sale_nft_origyn '(record {sale = record { token_id = "test 3"}; escrow_info = record { account = record {sub_account=blob "abcdefghijklmnopqrstuvxyz1234567"}}})'


// Do some work... then upgrade
dfx build gldt

dfx deploy gldt  --argument '(opt record {gldt_ledger_canister_id=principal "'$(dfx canister id ledger)'"; gldt_nft_canister_ids=vec{record { principal "'$(dfx identity get-principal)'"; record { grams=10}}}})' --mode=reinstall

*/
