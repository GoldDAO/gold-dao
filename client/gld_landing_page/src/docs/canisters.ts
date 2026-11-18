// Canister and neuron ids the docs refer to. They are the mainnet ids from
// canister_ids.json and sns_canister_ids.json at the repository root.

// Root canisters of the three SNS DAOs the docs link to on the ICP dashboard
export const snsRoots = {
  goldDao: 'tw2vt-hqaaa-aaaaq-aab6a-cai',
  origyn: 'leu43-oiaaa-aaaaq-aadgq-cai',
  waterNeuron: 'jmod6-4iaaa-aaaaq-aadkq-cai'
}

export const canisters = {
  snsRoot: snsRoots.goldDao,
  snsGovernance: 'tr3th-kiaaa-aaaaq-aab6q-cai',
  snsLedger: 'tyyy3-4aaaa-aaaaq-aab7a-cai',
  snsIndex: 'efv5g-kqaaa-aaaaq-aacaa-cai',
  snsArchive: 'fgzua-6iaaa-aaaaq-aacgq-cai',
  snsSwap: 't7z6p-ryaaa-aaaaq-aab7q-cai',
  app: 'rbsh4-yyaaa-aaaal-qdigq-cai',
  snsRewards: 'iyehc-lqaaa-aaaap-ab25a-cai',
  icpNeuron: 'j4jiq-sqaaa-aaaap-ab23a-cai',
  snsNeuronController: '54vkq-taaaa-aaaap-ahqra-cai',
  buybackBurn: 'atslz-hiaaa-aaaam-acq6q-cai'
}

export const neurons = {
  // NNS neuron held by the icp_neuron canister
  icp: '7446549063176501841',
  // ORIGYN and WaterNeuron neurons held by the sns_neuron_controller canister
  ogy: 'bf941a42ede5c1513b87375677e30fe6174a5f790be5850290182ebfa3b5f74d',
  wtn: 'cdeea7c0fcb8ded7c04ad21fbfdcd64347abfd6593e40d7e8e18f0e2b984f7ba',
  // GOLDAO neuron of the main contributing team. It votes by hand.
  team: '7eac04f2e207c04b8a7ac01b7505c715821ebbd7bc9815cef0a6842514f3b832'
}
