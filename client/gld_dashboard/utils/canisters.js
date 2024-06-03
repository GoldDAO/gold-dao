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

const isProd = process.env.ENV === 'production';

export const canisters = {
  governance: { canisterId: isProd ? 'tr3th-kiaaa-aaaaq-aab6q-cai' : 'j3ioe-7iaaa-aaaap-ab23q-cai', idlFactory: idlGovernance },
  // governanceStaging: { canisterId: 'j3ioe-7iaaa-aaaap-ab23q-cai', idlFactory: idlGovernance },
  archive: { canisterId: 'fgzua-6iaaa-aaaaq-aacgq-cai', idlFactory: idlArchive },
  ledger: { canisterId: isProd ? 'tyyy3-4aaaa-aaaaq-aab7a-cai' : 'irhm6-5yaaa-aaaap-ab24q-cai', idlFactory: idlLedger },
  root: { canisterId: isProd ? 'tw2vt-hqaaa-aaaaq-aab6a-cai' : 'i7fbw-giaaa-aaaap-ab25q-cai', idlFactory: idlRoot },
  ogy: { canisterId: isProd ? 'jwcfb-hyaaa-aaaaj-aac4q-cai' : 'vmxlg-4qaaa-aaaag-ak7gq-cai', idlFactory: idlLedger },
  icp: { canisterId: 'ryjl3-tyaaa-aaaaa-aaaba-cai', idlFactory: icp },
  lighthouseApi: { canisterId: 'zfp4v-oyaaa-aaaar-qadqq-cai', idlFactory: idlLighthouse },
  icpNeuron: { canisterId: isProd ? 'j4jiq-sqaaa-aaaap-ab23a-cai' : 'j2neh-vqaaa-aaaal-aduxq-cai', idlFactory: icpNeuron },
  icpSwap: { canisterId: 'moe7a-tiaaa-aaaag-qclfq-cai', idlFactory: icpSwap },
  snsRewards: { canisterId: isProd ? 'iyehc-lqaaa-aaaap-ab25a-cai' : '2f5ll-gqaaa-aaaak-qcfuq-cai', idlFactory: snsRewards },
  // snsRewardsStaging: { canisterId: '2f5ll-gqaaa-aaaak-qcfuq-cai', idlFactory: snsRewards }, // STAGING
  icpAllInfoNeuron: { canisterId: isProd ? 'j4jiq-sqaaa-aaaap-ab23a-cai' : 'j2neh-vqaaa-aaaal-aduxq-cai', idlFactory: icpAllInfoNeuron },
  // icpAllInfoNeuronStaging: {
  //   canisterId: 'j2neh-vqaaa-aaaal-aduxq-cai',
  //   idlFactory: icpAllInfoNeuron,
  // }, // STAGING
};
