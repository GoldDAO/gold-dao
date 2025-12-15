!#/bin/bash

## prepare metadata json

./scripts/_local/manual/origyn_icrc7_cmdlinetools \
  --network ic \
  --identity tmp.pem \
  --canister "g6yny-dyaaa-aaaab-qb2kq-cai" \
  upload-metadata metadata.json

## update metadata

dfx canister call --network staging gldnft_backend_1g update_nft_metadata '(
  record {
    token_id = 601 : nat;
    metadata = vec {
      record {
        "icrc97:metadata";
        variant {
          Array = vec {
            variant {
              Text = "https://mgej7-baaaa-aaaab-qcyqa-cai.raw.icp0.io/420d4f9db845237bda17a580892ae400c80f435e91b543b43e91a0ecd57769f1.json"
            };
          }
        };
      };
    };
  },
)'

## dfx mint - metadata not correct
dfx canister call --network staging gldnft_backend_1g mint '(
  record {
    mint_requests = vec {
      record {
        metadata = vec {
          record {
            "icrc97:metadata";
            variant {
              Array = vec {
                variant {
                  Text = "https://mgej7-baaaa-aaaab-qcyqa-cai.raw.icp0.io/420d4f9db845237bda17a580892ae400c80f435e91b543b43e91a0ecd57769f1.json"
                };
              }
            };
          };
        };
        memo = null;
        token_owner = record {
          owner = principal "gt3tu-ptchj-pyoh6-uldtw-2wzh6-hjm7e-o7chg-3jj64-wxvwd-l2nvp-wqe";
          subaccount = null;
        };
      };
    };
  },
)'
