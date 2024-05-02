import icp from './idls/icp';
import icpAllInfoNeuron from './idls/icpAllInfoNeuron';
import icpNeuron from './idls/icpNeuron';
import icpSwap from './idls/icpSwap';
import { idlFactory as idlArchive } from './idls/archive';
import { idlFactory as idlGovernance } from './idls/governance';
import { idlFactory as idlLedger } from './idls/ledger';
import idlLighthouse from './idls/lighthouseApi';
import idlRoot from './idls/root';
import snsRewards from './idls/snsRewards';

// eslint-disable-next-line import/prefer-default-export
export const canisters = {
  governance: { canisterId: 'tr3th-kiaaa-aaaaq-aab6q-cai', idlFactory: idlGovernance },
  governanceStaging: { canisterId: 'j3ioe-7iaaa-aaaap-ab23q-cai', idlFactory: idlGovernance },
  archive: { canisterId: 'fgzua-6iaaa-aaaaq-aacgq-cai', idlFactory: idlArchive },
  ledger: { canisterId: 'tyyy3-4aaaa-aaaaq-aab7a-cai', idlFactory: idlLedger },
  root: { canisterId: 'tw2vt-hqaaa-aaaaq-aab6a-cai', idlFactory: idlRoot },
  ogy: { canisterId: 'jwcfb-hyaaa-aaaaj-aac4q-cai', idlFactory: idlLedger },
  icp: { canisterId: 'ryjl3-tyaaa-aaaaa-aaaba-cai', idlFactory: icp },
  lighthouseApi: { canisterId: 'zfp4v-oyaaa-aaaar-qadqq-cai', idlFactory: idlLighthouse },
  icpNeuron: { canisterId: 'j4jiq-sqaaa-aaaap-ab23a-cai', idlFactory: icpNeuron },
  icpSwap: { canisterId: 'moe7a-tiaaa-aaaag-qclfq-cai', idlFactory: icpSwap },
  snsRewards: { canisterId: 'iyehc-lqaaa-aaaap-ab25a-cai', idlFactory: snsRewards },
  snsRewardsStaging: { canisterId: '2f5ll-gqaaa-aaaak-qcfuq-cai', idlFactory: snsRewards }, // STAGING
  icpAllInfoNeuron: { canisterId: 'j4jiq-sqaaa-aaaap-ab23a-cai', idlFactory: icpAllInfoNeuron },
  icpAllInfoNeuronStaging: {
    canisterId: 'j2neh-vqaaa-aaaal-aduxq-cai',
    idlFactory: icpAllInfoNeuron,
  }, // STAGING
};
