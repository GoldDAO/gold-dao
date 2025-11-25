use crate::client::origyn_nft::icrc37_approve_tokens;
use crate::client::origyn_nft::icrc7_balance_of;
use crate::client::origyn_nft::icrc7_owner_of;
use crate::origyn_nft_test_env::nft_utils::mint_nft;
use crate::origyn_nft_test_env::nft_utils::setup_core_canister;
use crate::utils::random_principal;
use crate::utils::tick_n_blocks;
use bity_ic_canister_time::MINUTE_IN_MS;
use candid::Nat;
use candid::Principal;
use icrc_ledger_types::icrc::generic_value::ICRC3Value;
use icrc_ledger_types::icrc1::account::Account;
use origyn_nft_canister::Args;
use origyn_nft_canister::InitArgs;
use pocket_ic::PocketIc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use types::CanisterId;

pub struct OrigynNftTestEnv {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,
    pub nft_owner1: Principal,
    pub nft_owner2: Principal,
    pub collection_canister_id: CanisterId,
    pub minted_nfts: HashMap<Nat, NftInfo>,
}

pub struct NftInfo {
    pub id: Nat,
    pub owner: Principal,
    metadata: Vec<(String, ICRC3Value)>,
}

impl OrigynNftTestEnv {
    pub fn balance_of(&mut self, owner: Principal) -> Vec<Nat> {
        let pic = self.pic.borrow();
        icrc7_balance_of(
            &pic,
            owner.into(),
            self.collection_canister_id,
            &vec![owner.into()],
        )
    }

    pub fn owner_of(
        &mut self,
        token_id: &Nat,
    ) -> std::vec::Vec<std::option::Option<icrc_ledger_types::icrc1::account::Account>> {
        let pic = self.pic.borrow();
        icrc7_owner_of(
            &pic,
            self.controller,
            self.collection_canister_id,
            &vec![token_id.clone()],
        )
    }

    pub fn mint_nft(&mut self, owner: Principal, metadata: Vec<(String, ICRC3Value)>) -> Nat {
        let pic = self.pic.borrow();
        let response = match mint_nft(
            &pic,
            owner.into(),
            self.controller,
            self.collection_canister_id,
            metadata.clone(),
        ) {
            Ok(nft_id) => {
                self.minted_nfts.insert(
                    nft_id.clone(),
                    NftInfo {
                        id: nft_id.clone(),
                        owner,
                        metadata: metadata.clone(),
                    },
                );
                nft_id
            }
            Err(e) => panic!("Failed to mint NFT: {:?}", e),
        };
        tick_n_blocks(&pic, 10);
        response
    }

    pub fn approve(&mut self, owner: Principal, token_id: Nat, to: Principal) -> Nat {
        let pic = self.pic.borrow();

        let current_time = pic.get_time().as_nanos_since_unix_epoch();
        let approval_info = origyn_nft_canister::ApprovalInfo {
            spender: Account {
                owner: to,
                subaccount: None,
            },
            from_subaccount: None,
            expires_at: None,
            memo: None,
            created_at_time: current_time,
        };

        let approve_args = vec![
            origyn_nft_canister::icrc37_approve_tokens::ApproveTokenArg {
                token_id: token_id.clone(),
                approval_info: approval_info.clone(),
            },
        ];

        let response =
            match icrc37_approve_tokens(&pic, owner, self.collection_canister_id, &approve_args) {
                Ok(_) => token_id,
                Err(e) => panic!("Failed to approve NFT: {:?}", e),
            };
        tick_n_blocks(&pic, 10);

        response
    }
}

pub struct OrigynNftTestEnvBuilder {
    pub pic: Rc<RefCell<PocketIc>>,
    pub controller: Principal,

    nft_owner1: Principal,
    nft_owner2: Principal,
    collection_id: CanisterId,
}

impl Default for OrigynNftTestEnvBuilder {
    fn default() -> Self {
        OrigynNftTestEnvBuilder {
            pic: Rc::new(RefCell::new(PocketIc::default())),
            controller: Principal::anonymous(),
            nft_owner1: random_principal(),
            nft_owner2: random_principal(),
            collection_id: Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        }
    }
}

impl OrigynNftTestEnvBuilder {
    pub fn new(pic: &Rc<RefCell<PocketIc>>, controller: Principal) -> Self {
        OrigynNftTestEnvBuilder {
            pic: pic.clone(),
            controller,
            ..Default::default()
        }
    }

    pub fn with_controller(mut self, principal: Principal) -> Self {
        self.controller = principal;
        self
    }

    pub fn build(&mut self, init_args: InitArgs) -> OrigynNftTestEnv {
        println!("Start building OrigynNftTestEnv");
        let pic = self.pic.borrow();

        self.collection_id = pic.create_canister_with_settings(Some(self.controller.clone()), None);

        pic.tick();
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 10));

        println!("collection_id: {}", self.collection_id.to_text());

        let nft_init_args = Args::Init(init_args);

        let collection_canister_id =
            setup_core_canister(&pic, self.collection_id, nft_init_args, self.controller);

        pic.tick();
        pic.advance_time(Duration::from_millis(MINUTE_IN_MS * 30));

        println!(
            "collection_canister_id: {}",
            collection_canister_id.to_text()
        );

        let pic = Rc::clone(&self.pic);
        OrigynNftTestEnv {
            controller: self.controller,
            nft_owner1: self.nft_owner1,
            nft_owner2: self.nft_owner2,
            collection_canister_id: collection_canister_id,
            minted_nfts: HashMap::new(),
            pic,
        }
    }
}
