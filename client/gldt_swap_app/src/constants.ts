export const APP_MODE = import.meta.env.MODE;
export const GLDT_SWAP_APP_FRONT_URL = import.meta.env
  .VITE_GLDT_SWAP_APP_FRONT_URL;
export const GLD_NFT_1G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_1G_CANISTER_ID;
export const GLD_NFT_10G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_10G_CANISTER_ID;
export const GLD_NFT_100G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_100G_CANISTER_ID;
export const GLD_NFT_1000G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_1000G_CANISTER_ID;
export const OGY_LEDGER_CANISTER_ID = import.meta.env
  .VITE_OGY_LEDGER_CANISTER_ID;
export const SWAP_CANISTER_ID = import.meta.env.VITE_SWAP_CANISTER_ID;
export const GLDT_LEDGER_CANISTER_ID = import.meta.env
  .VITE_GLDT_LEDGER_CANISTER_ID;

export const API_OGY_BASE_URL="https://api.origyn.com"

export const GLDT_TX_FEE = 10000000000;
export const GLDT_FORWARD_SWAP_FEE = 10000000;
export const GLDT_REVERSE_SWAP_FEE = 100000000;
export const GLDT_VALUE_1G_NFT = 100;

if (!(APP_MODE === "production")) {
  console.log(`APP_MODE=${APP_MODE}`);

  if (!GLDT_SWAP_APP_FRONT_URL)
    console.log(
      "No GLDT_SWAP_APP_FRONT_URL environment variable. Set GLDT_SWAP_APP_FRONT_URL environment variable."
    );
  else console.log(`GLDT_SWAP_APP_FRONT_URL=${GLDT_SWAP_APP_FRONT_URL}`);

  if (!GLD_NFT_1G_CANISTER_ID)
    console.log(
      "No GLD_NFT_1G_CANISTER_ID environment variable. Set GLD_NFT_1G_CANISTER_ID environment variable."
    );
  else console.log(`GLD_NFT_1G_CANISTER_ID=${GLD_NFT_1G_CANISTER_ID}`);

  if (!GLD_NFT_10G_CANISTER_ID)
    console.log(
      "No GLD_NFT_10G_CANISTER_ID environment variable. Set GLD_NFT_10G_CANISTER_ID environment variable."
    );
  else console.log(`GLD_NFT_10G_CANISTER_ID=${GLD_NFT_10G_CANISTER_ID}`);

  if (!GLD_NFT_100G_CANISTER_ID)
    console.log(
      "No GLD_NFT_100G_CANISTER_ID environment variable. Set GLD_NFT_100G_CANISTER_ID environment variable."
    );
  else console.log(`GLD_NFT_100G_CANISTER_ID=${GLD_NFT_100G_CANISTER_ID}`);

  if (!GLD_NFT_1000G_CANISTER_ID)
    console.log(
      "No GLD_NFT_1000G_CANISTER_ID environment variable. Set GLD_NFT_1000G_CANISTER_ID environment variable."
    );
  else console.log(`GLD_NFT_1000G_CANISTER_ID=${GLD_NFT_1000G_CANISTER_ID}`);

  if (!OGY_LEDGER_CANISTER_ID)
    console.log(
      "No OGY_LEDGER_CANISTER_ID environment variable. Set OGY_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`OGY_LEDGER_CANISTER_ID=${OGY_LEDGER_CANISTER_ID}`);

  if (!SWAP_CANISTER_ID)
    console.log(
      "No SWAP_CANISTER_ID environment variable. Set SWAP_CANISTER_ID environment variable."
    );
  else console.log(`SWAP_CANISTER_ID=${SWAP_CANISTER_ID}`);

  if (!GLDT_LEDGER_CANISTER_ID)
    console.log(
      "No GLDT_LEDGER_CANISTER_ID environment variable. Set GLDT_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`GLDT_LEDGER_CANISTER_ID=${GLDT_LEDGER_CANISTER_ID}`);
}
