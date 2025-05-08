export const APP_MODE = import.meta.env.MODE;

export const GLD_NFT_1G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_1G_CANISTER_ID;
export const GLD_NFT_10G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_10G_CANISTER_ID;
export const GLD_NFT_100G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_100G_CANISTER_ID;
export const GLD_NFT_1000G_CANISTER_ID = import.meta.env
  .VITE_GLD_NFT_1000G_CANISTER_ID;

export const GOLDAO_LEDGER_CANISTER_ID = import.meta.env
  .VITE_GOLDAO_LEDGER_CANISTER_ID;
export const GOLDAO_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_GOLDAO_LEDGER_INDEX_CANISTER_ID;
export const OGY_LEDGER_CANISTER_ID = import.meta.env
  .VITE_OGY_LEDGER_CANISTER_ID;
export const OGY_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_OGY_LEDGER_INDEX_CANISTER_ID;
export const GLDT_LEDGER_CANISTER_ID = import.meta.env
  .VITE_GLDT_LEDGER_CANISTER_ID;
export const GLDT_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_GLDT_LEDGER_INDEX_CANISTER_ID;
export const ICP_LEDGER_CANISTER_ID = import.meta.env
  .VITE_ICP_LEDGER_CANISTER_ID;
export const ICP_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_ICP_LEDGER_INDEX_CANISTER_ID;
export const WTN_LEDGER_CANISTER_ID = import.meta.env
  .VITE_WTN_LEDGER_CANISTER_ID;
export const WTN_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_WTN_LEDGER_INDEX_CANISTER_ID;
export const CKUSDT_LEDGER_CANISTER_ID = import.meta.env
  .VITE_CKUSDT_LEDGER_CANISTER_ID;
export const CKUSDT_LEDGER_INDEX_CANISTER_ID = import.meta.env
  .VITE_CKUSDT_LEDGER_INDEX_CANISTER_ID;

export const SWAP_CANISTER_ID = import.meta.env.VITE_SWAP_CANISTER_ID;
export const SNS_NEURONS_ICP_CANISTER_ID = import.meta.env
  .VITE_SNS_NEURONS_ICP_CANISTER_ID;
export const SNS_NEURONS_OGY_CANISTER_ID = import.meta.env
  .VITE_SNS_NEURONS_OGY_CANISTER_ID;
export const SNS_ROOT_CANISTER_ID = import.meta.env
  .VITE_SNS_ROOT_CANISTER_ID;
export const SNS_GOVERNANCE_CANISTER_ID = import.meta.env
  .VITE_SNS_GOVERNANCE_CANISTER_ID;
export const SNS_REWARDS_CANISTER_ID = import.meta.env
  .VITE_SNS_REWARDS_CANISTER_ID;
export const SNS_SUPER_STATS_CANISTER_ID = import.meta.env
  .VITE_SNS_SUPER_STATS_CANISTER_ID;
export const GLDT_STAKE_CANISTER_ID = import.meta.env
  .VITE_GLDT_STAKE_CANISTER_ID;

export const BITY_GOLD_API_BASE_URL = import.meta.env.VITE_BITY_GOLD_API_BASE_URL;

export const ICP_ICRC_API_BASE_URL = "https://icrc-api.internetcomputer.org/api/v1"
export const OGY_API_BASE_URL = "https://api.origyn.com"
export const IC_EXPLORER_API_BASE_URL = "https://api.icexplorer.io/api"

export const GOLDAO_LEDGER_CANISTER_ID_IC = "tyyy3-4aaaa-aaaaq-aab7a-cai"
export const GLDT_LEDGER_CANISTER_ID_IC = "6c7su-kiaaa-aaaar-qaira-cai"
export const ICP_LEDGER_CANISTER_ID_IC = "ryjl3-tyaaa-aaaaa-aaaba-cai"
export const CK_USDC_LEDGER_CANISTER_ID_IC = "xevnm-gaaaa-aaaar-qafnq-cai"
export const SNS_ROOT_CANISTER_ID_IC = "tw2vt-hqaaa-aaaaq-aab6a-cai"
export const SNS_GOVERNANCE_CANISTER_ID_IC = "tr3th-kiaaa-aaaaq-aab6q-cai"
export const ROOT_ACCOUNT_GLDGOV = "tr3th-kiaaa-aaaaq-aab6q-cai-nif4qry.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8"
export const ICPSWAP_CANISTER_ID = "moe7a-tiaaa-aaaag-qclfq-cai";
export const SWAP_POOL_ICP_GLDT_CANISTER_ID_IC = "4omhz-yiaaa-aaaag-qnalq-cai";
export const KONGSWAP_CANISTER_ID_IC = "2ipq2-uqaaa-aaaar-qailq-cai";

export const REVERSE_GLDT_TX_FEE = 100000000;
export const GLDT_VALUE_1G_NFT = 100;
export const MAX_SWAP_SLIPPAGE = 5;

if (!(APP_MODE === "production")) {
  console.log(`APP_MODE=${APP_MODE}`);

  if (!BITY_GOLD_API_BASE_URL)
    console.log(
      "No BITY_GOLD_API_BASE_URL environment variable. Set BITY_GOLD_API_BASE_URL environment variable."
    );
  else console.log(`BITY_GOLD_API_BASE_URL=${BITY_GOLD_API_BASE_URL}`);

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

  if (!GOLDAO_LEDGER_CANISTER_ID)
    console.log(
      "No GOLDAO_LEDGER_CANISTER_ID environment variable. Set GOLDAO_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`GOLDAO_LEDGER_CANISTER_ID=${GOLDAO_LEDGER_CANISTER_ID}`);

  if (!GOLDAO_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No GOLDAO_LEDGER_INDEX_CANISTER_ID environment variable. Set GOLDAO_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`GOLDAO_LEDGER_INDEX_CANISTER_ID=${GOLDAO_LEDGER_INDEX_CANISTER_ID}`);

  if (!OGY_LEDGER_CANISTER_ID)
    console.log(
      "No OGY_LEDGER_CANISTER_ID environment variable. Set OGY_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`OGY_LEDGER_CANISTER_ID=${OGY_LEDGER_CANISTER_ID}`);

  if (!OGY_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No OGY_LEDGER_INDEX_CANISTER_ID environment variable. Set OGY_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`OGY_LEDGER_INDEX_CANISTER_ID=${OGY_LEDGER_INDEX_CANISTER_ID}`);

  if (!GLDT_LEDGER_CANISTER_ID)
    console.log(
      "No GLDT_LEDGER_CANISTER_ID environment variable. Set GLDT_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`GLDT_LEDGER_CANISTER_ID=${GLDT_LEDGER_CANISTER_ID}`);

  if (!GLDT_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No GLDT_LEDGER_INDEX_CANISTER_ID environment variable. Set GLDT_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`GLDT_LEDGER_INDEX_CANISTER_ID=${GLDT_LEDGER_INDEX_CANISTER_ID}`);

  if (!WTN_LEDGER_CANISTER_ID)
    console.log(
      "No WTN_LEDGER_CANISTER_ID environment variable. Set WTN_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`WTN_LEDGER_CANISTER_ID=${WTN_LEDGER_CANISTER_ID}`);

  if (!WTN_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No WTN_LEDGER_INDEX_CANISTER_ID environment variable. Set WTN_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`WTN_LEDGER_INDEX_CANISTER_ID=${WTN_LEDGER_INDEX_CANISTER_ID}`);

  if (!CKUSDT_LEDGER_CANISTER_ID)
    console.log(
      "No CKUSDT_LEDGER_CANISTER_ID environment variable. Set CKUSDT_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`CKUSDT_LEDGER_CANISTER_ID=${CKUSDT_LEDGER_CANISTER_ID}`);

  if (!CKUSDT_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No CKUSDT_LEDGER_INDEX_CANISTER_ID environment variable. Set CKUSDT_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`CKUSDT_LEDGER_INDEX_CANISTER_ID=${CKUSDT_LEDGER_INDEX_CANISTER_ID}`);

  if (!ICP_LEDGER_CANISTER_ID)
    console.log(
      "No ICP_LEDGER_CANISTER_ID environment variable. Set ICP_LEDGER_CANISTER_ID environment variable."
    );
  else console.log(`ICP_LEDGER_CANISTER_ID=${ICP_LEDGER_CANISTER_ID}`);

  if (!ICP_LEDGER_INDEX_CANISTER_ID)
    console.log(
      "No ICP_LEDGER_INDEX_CANISTER_ID environment variable. Set ICP_LEDGER_INDEX_CANISTER_ID environment variable."
    );
  else console.log(`ICP_LEDGER_INDEX_CANISTER_ID=${ICP_LEDGER_INDEX_CANISTER_ID}`);

  if (!SWAP_CANISTER_ID)
    console.log(
      "No SWAP_CANISTER_ID environment variable. Set SWAP_CANISTER_ID environment variable."
    );
  else console.log(`SWAP_CANISTER_ID=${SWAP_CANISTER_ID}`);

  if (!SNS_NEURONS_ICP_CANISTER_ID)
    console.log(
      "No SNS_NEURONS_ICP_CANISTER_ID environment variable. Set SNS_NEURONS_ICP_CANISTER_ID environment variable."
    );
  else console.log(`SNS_NEURONS_ICP_CANISTER_ID=${SNS_NEURONS_ICP_CANISTER_ID}`);

  if (!SNS_NEURONS_OGY_CANISTER_ID)
    console.log(
      "No SNS_NEURONS_OGY_CANISTER_ID environment variable. Set SNS_NEURONS_OGY_CANISTER_ID environment variable."
    );
  else console.log(`SNS_NEURONS_OGY_CANISTER_ID=${SNS_NEURONS_OGY_CANISTER_ID}`);

  if (!SNS_ROOT_CANISTER_ID)
    console.log(
      "No SNS_ROOT_CANISTER_ID environment variable. Set SNS_ROOT_CANISTER_ID environment variable."
    );
  else console.log(`SNS_ROOT_CANISTER_ID=${SNS_ROOT_CANISTER_ID}`);

  if (!SNS_GOVERNANCE_CANISTER_ID)
    console.log(
      "No SNS_GOVERNANCE_CANISTER_ID environment variable. Set SNS_GOVERNANCE_CANISTER_ID environment variable."
    );
  else console.log(`SNS_GOVERNANCE_CANISTER_ID=${SNS_GOVERNANCE_CANISTER_ID}`);

  if (!SNS_REWARDS_CANISTER_ID)
    console.log(
      "No SNS_REWARDS_CANISTER_ID environment variable. Set SNS_REWARDS_CANISTER_ID environment variable."
    );

  if (!SNS_SUPER_STATS_CANISTER_ID)
    console.log(
      "No SNS_SUPER_STATS_CANISTER_ID environment variable. Set SNS_SUPER_STATS_CANISTER_ID environment variable."
    );
  else console.log(`SNS_SUPER_STATS_CANISTER_ID=${SNS_SUPER_STATS_CANISTER_ID}`);

  if (!GLDT_STAKE_CANISTER_ID)
    console.log(
      "No GLDT_STAKE_CANISTER_ID environment variable. Set GLDT_STAKE_CANISTER_ID environment variable."
    );
  else console.log(`GLDT_STAKE_CANISTER_ID=${GLDT_STAKE_CANISTER_ID}`);

  console.log(`ICPSWAP_CANISTER_ID=${ICPSWAP_CANISTER_ID}`)
  console.log(`KONGSWAP_CANISTER_ID_IC=${KONGSWAP_CANISTER_ID_IC}`)
  console.log(`GOLDAO_LEDGER_CANISTER_ID_IC=${GOLDAO_LEDGER_CANISTER_ID_IC}`)
  console.log(`GLDT_LEDGER_CANISTER_ID_IC=${GLDT_LEDGER_CANISTER_ID_IC}`)
  console.log(`ICP_LEDGER_CANISTER_ID_IC=${ICP_LEDGER_CANISTER_ID_IC}`)
  console.log(`CK_USDC_LEDGER_CANISTER_ID_IC=${CK_USDC_LEDGER_CANISTER_ID_IC}`)
  console.log(`SNS_ROOT_CANISTER_ID_IC=${SNS_ROOT_CANISTER_ID_IC}`)
  console.log(`SNS_GOVERNANCE_CANISTER_ID_IC=${SNS_GOVERNANCE_CANISTER_ID_IC}`)
}
