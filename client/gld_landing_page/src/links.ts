// Every outgoing link of the site. Links between its own pages come from
// src/docs/nav.ts.

// Gold DAO SNS root canister, used by the NNS dapp and the ICP dashboard URLs
const SNS_ROOT = 'tw2vt-hqaaa-aaaaq-aab6a-cai'
const NNS_DAPP = 'https://nns.ic0.app'
const ICP_DASHBOARD = 'https://dashboard.internetcomputer.org'
const APP = 'https://app.gold-dao.org'
// Public mirror of this repository
const GITHUB = 'https://github.com/GoldDAO/gold-dao'

export const links = {
  app: APP,
  appGovern: `${APP}/govern`,
  appWallet: `${APP}/wallet`,
  appEarn: `${APP}/earn`,
  proposals: `${NNS_DAPP}/proposals/?u=${SNS_ROOT}`,
  icpDashboard: `${ICP_DASHBOARD}/sns/${SNS_ROOT}`,
  x: 'https://x.com/goldaoapp',
  github: GITHUB,
  // The canister ids of every network, on the branch that production is built from
  githubCanisterIds: `${GITHUB}/blob/master/canister_ids.json`,
  nnsDapp: NNS_DAPP,
  // Neuron staking page of the NNS dapp with Gold DAO selected
  nnsNeurons: `${NNS_DAPP}/neurons/?u=${SNS_ROOT}`,
  // ICP to GOLDAO, by ledger canister id
  icpSwap:
    'https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=tyyy3-4aaaa-aaaaq-aab7a-cai',
  // Background paper of proposal 317, served by the gold_dao_chapter_2 canister
  chapter2: 'https://fjdhj-uyaaa-aaaai-q4fma-cai.icp0.io/GoldDAO_Chapter2.0.pdf',
  gldt: 'https://gldt.org'
}

// A Gold DAO proposal in the NNS dapp
export const proposalUrl = (id: number) =>
  `${NNS_DAPP}/proposal/?u=${SNS_ROOT}&proposal=${id}`

// Pages of the ICP dashboard
export const canisterUrl = (canisterId: string) =>
  `${ICP_DASHBOARD}/canister/${canisterId}`

export const icpAccountUrl = (accountId: string) =>
  `${ICP_DASHBOARD}/account/${accountId}`

export const nnsNeuronUrl = (neuronId: string) =>
  `${ICP_DASHBOARD}/neuron/${neuronId}`

export const snsNeuronUrl = (snsRoot: string, neuronId: string) =>
  `${ICP_DASHBOARD}/sns/${snsRoot}/neuron/${neuronId}`

// An account of the GOLDAO ledger, in the textual ICRC-1 format
export const goldaoAccountUrl = (account: string) =>
  `${ICP_DASHBOARD}/sns/${SNS_ROOT}/account/${account}`
