import { idlFactory as idlIcp } from './idls/icp';
import { idlFactory as icpAllInfoNeuron } from './idls/icpAllInfoNeuron';
import { idlFactory as icpNeuron } from './idls/icpNeuron';
import { idlFactory as ogyNeurons } from './idls/ogyNeurons';
import { idlFactory as icpSwap } from './idls/icpSwap';
import { idlFactory as idlArchive } from './idls/archive';
import { idlFactory as idlGovernance } from './idls/governance';
import { idlFactory as idlLedger } from './idls/ledger';
import { idlFactory as idlLighthouse } from './idls/lighthouseApi';
import { idlFactory as idlRoot } from './idls/root';
import { idlFactory as snsRewards } from './idls/snsRewards';
import { idlFactory as idlManagement } from './idls/management';

// eslint-disable-next-line import/prefer-default-export

const isProd = process.env.ENV === 'prod' || process.env.ENV === 'preprod';

const canisters = {
  governance: { canisterId: isProd ? 'tr3th-kiaaa-aaaaq-aab6q-cai' : 'j3ioe-7iaaa-aaaap-ab23q-cai', idlFactory: idlGovernance },
  archive: { canisterId: 'fgzua-6iaaa-aaaaq-aacgq-cai', idlFactory: idlArchive },
  ledger: { canisterId: isProd ? 'tyyy3-4aaaa-aaaaq-aab7a-cai' : 'irhm6-5yaaa-aaaap-ab24q-cai', idlFactory: idlLedger },
  root: { canisterId: isProd ? 'tw2vt-hqaaa-aaaaq-aab6a-cai' : 'i7fbw-giaaa-aaaap-ab25q-cai', idlFactory: idlRoot },
  ogy: { canisterId: isProd ? 'jwcfb-hyaaa-aaaaj-aac4q-cai' : 'vmxlg-4qaaa-aaaag-ak7gq-cai', idlFactory: idlLedger },
  icp: { canisterId: 'ryjl3-tyaaa-aaaaa-aaaba-cai', idlFactory: idlIcp },
  lighthouseApi: { canisterId: 'zfp4v-oyaaa-aaaar-qadqq-cai', idlFactory: idlLighthouse },
  icpNeuron: { canisterId: isProd ? 'j4jiq-sqaaa-aaaap-ab23a-cai' : 'j2neh-vqaaa-aaaal-aduxq-cai', idlFactory: icpNeuron },
  ogyNeuron: { canisterId: '54vkq-taaaa-aaaap-ahqra-cai', idlFactory: ogyNeurons },
  icpSwap: { canisterId: 'moe7a-tiaaa-aaaag-qclfq-cai', idlFactory: icpSwap },
  snsRewards: { canisterId: isProd ? 'iyehc-lqaaa-aaaap-ab25a-cai' : '2f5ll-gqaaa-aaaak-qcfuq-cai', idlFactory: snsRewards },
  icpAllInfoNeuron: { canisterId: isProd ? 'j4jiq-sqaaa-aaaap-ab23a-cai' : 'j2neh-vqaaa-aaaal-aduxq-cai', idlFactory: icpAllInfoNeuron },
  management: { canisterId: isProd ? 'g5je6-yaaaa-aaaap-ahkza-cai' : 't5uzw-sqaaa-aaaan-qmoaq-cai', idlFactory: idlManagement },
};

export default canisters;
