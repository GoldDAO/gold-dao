# Deploy scripts for staging deployment

# 1. ONLY NEEDED FOR LOCAL DEVELOPMENT, FOR STAGING / PROD, start with step 2
# create gldt core canister. Needed to deploy gldt_ledger to assign the minting account.
dfx canister create gldt_core

# Replace "--network staging" with "--network local" for local development
# 2. deploy gldt ledger
# dfx deploy gldt_ledger --network local --argument '(record{minting_account="'"$(dfx ledger account-id --network local --of-canister gldt_core)"'"; send_whitelist=vec{}; initial_values=vec{}})'
# dfx deploy gldt_ledger --network staging --argument '(record{minting_account="'"$(dfx ledger account-id --network staging --of-canister gldt_core)"'"; send_whitelist=vec{}; initial_values=vec{}})'
./scripts/deploy-ledger.sh


# 3. deploy gldt core
dfx deploy gldt_core --network staging --argument '(opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network staging gldt_ledger)"'";
  gld_nft_canister_ids=vec{
    record { principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'"; record { grams=1}};
    record { principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'"; record { grams=10}};
    record { principal "'"$(dfx identity get-principal)"'"; record { grams=100}};
    }})'
dfx deploy gldt_core --argument '(opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network staging gldt_ledger)"'";
  gld_nft_canister_ids=vec{
    record { principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'"; record { grams=1}}
  }})'

# 4. deploy gldt frontend
dfx deploy gldt_swap_app --network local
dfx deploy gldt_swap_app --network staging

####################
### KYC Canister ###
####################
# 5. deploy yumi kyc canister & adding gldt as whitelisted canister
dfx deploy yumi_kyc --network staging --argument '(principal "'"$(dfx identity get-principal)"'")'
## a. add KYC access level for gold
dfx canister call --network staging yumi_kyc setKycAccess '(variant {Gold}, variant {Tier1} ,variant{Pass} )' &&
dfx canister call --network staging yumi_kyc setKycAccess '(variant {Gold}, variant {Tier2} ,variant{Pass} )'  &&
dfx canister call --network staging yumi_kyc setKycAccess '(variant {Gold}, variant {Tier3} ,variant{Pass} )'
dfx canister call --network staging yumi_kyc getAllKycAccess
## b. update KYC status of the GLDT canister
dfx canister call --network staging yumi_kyc batch_update_kyc_status '(vec {
  record { kycLevel= variant {Tier3}; account = variant {ICRC1 = record { owner = principal "'"$(dfx canister id --network staging gldt_core)"'"; }}}
  })'
# c. Add your wallets to KYC canister
dfx canister call --network staging yumi_kyc batch_update_kyc_status '(vec {
  record { kycLevel= variant {Tier3}; account = variant {ICRC1 = record { owner = principal "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe"; }}};
  record { kycLevel= variant {Tier3}; account = variant {ICRC1 = record { owner = principal "xtjhb-skaw3-4ljsp-34fxx-epade-cbgz5-eq3dc-zvr3t-mdywz-ueocs-5qe"; }}}
  })'
dfx canister call --network staging yumi_kyc getAllKycStatus
# d. set the routers for gold channels for the GLD NFT canisters
dfx canister call --network staging yumi_kyc setRouter '(principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'", variant{Gold} )' &&
dfx canister call --network staging yumi_kyc setRouter '(principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'", variant{Gold} )' &&
dfx canister call --network staging yumi_kyc setRouter '(principal "'"$(dfx identity get-principal)"'", variant{Gold} )'
dfx canister call --network staging yumi_kyc getAllRouters
## e. verify kyc response of trading request
dfx canister call --network staging yumi_kyc icrc17_kyc_request '(record {counterparty = variant {ICRC1 = record { owner = principal "'"$(dfx canister id --network staging gldnft_backend_1g)"'"; }}})'




dfx canister call --network local yumi_kyc  "'"$(dfx ledger account-id --network local --of-canister gldt_core)"'"
dfx canister call --network local yumi_kyc getCustomerKyc
dfx canister call --network local yumi_kyc batch_update_kyc_status '(vec { record { kycLevel= variant {Full}; account = variant {ICRC1 = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; }}}})'
dfx canister call --network local yumi_kyc addSubmitKyc '(variant {ICRC1 = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; }})'
dfx canister call --network local yumi_kyc getKycStatus '(variant {ICRC1 = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; }})'
dfx canister call --network local yumi_kyc icrc17_kyc_request_for_channel '(variant {Gold}, record {counterparty = variant {ICRC1 = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; }}})'
dfx canister call --network local yumi_kyc icrc17_kyc_request '(record {counterparty = variant {ICRC1 = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; }}})'
dfx canister call --network local yumi_kyc getAllRouters
dfx canister call --network local yumi_kyc setRouter '(principal "jf4o7-6zzxo-5n6ru-k7dg2-pkyl2-jmnhi-frwzq-anevx-7b5si-spceg-pae", variant{Gold} )'



# test calls

dfx canister call gldt_core --network staging nft_info '(record {source_canister = principal "obapm-2iaaa-aaaak-qcgca-cai"; nft_id = "gold-013052";})'
dfx canister call gldt_core --network local request_offer '(record {nft_id = "gold-067883"; to_subaccount = vec {0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0}; requested_memo = 0})'
dfx canister call gldt_core --network staging notify_sale_nft_origyn '(record {sale = record { token_id = "gold_1234"}; escrow_info = record { account = record {sub_account=blob "abcdefghijklmnopqrstuvxyz1234567"}}})'




dfx canister call --network staging gldt_ledger icrc1_balance_of '(record {owner = principal "'"$(dfx canister id --network staging gldt_core)"'";})'
dfx canister call --network staging gldt_ledger icrc1_balance_of '(record {owner = principal "'"$(dfx identity get-principal)"'";})'
dfx canister call --network staging gldt_ledger icrc1_balance_of '(record {owner = principal "'"$(dfx identity get-principal)"'"; subaccount = opt blob "abcdefghijklmnopqrstuvxyz1234567"})'

dfx canister call --network staging gldt_ledger icrc1_balance_of '(record {owner = principal "ccjse-eaaaa-aaaao-a2ixq-cai";})'
dfx canister call --network staging gldt_ledger icrc1_transfer '(record {
  to = record {owner = principal "ccjse-eaaaa-aaaao-a2ixq-cai";};
  amount = 100_000_000_000;
  })'
# ccjse-eaaaa-aaaao-a2ixq-cai
    #[serde(default)]
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    #[serde(default)]
    pub fee: Option<NumTokens>,
    #[serde(default)]
    pub created_at_time: Option<u64>,
    #[serde(default)]
    pub memo: Option<Memo>,
    pub amount: NumTokens,

dfx canister call --network staging gldt_ledger get_blocks '(record {start= record { blocks= vec {0}} ;length=2})'
dfx canister call --network staging gldt_ledger get_data_certificate
dfx canister call --network staging gldt_ledger icrc1_balance_of '(record { owner = principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'"; subaccount = opt blob "?–üÏYM<ðz·l°Û_ðf<ÍÈAW‹¾âhŽ"})'
dfx canister call --network staging gldt_ledger icrc1_balance_of '(record { owner = principal "'"$(dfx canister id --network staging gldnft_backend_10g)"'";})'


# Notes
#
# 1. Generate .did file for gldt_core canister
#   Since gldt_core is written in rust, the .did file needs to be generated manually.
#   run the script `./scripts/generate-did.sh` to generate the .did file and
#   automatically create the js and ts bindings.
