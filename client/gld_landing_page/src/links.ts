// Gold DAO SNS root canister, used by the NNS dapp and the ICP dashboard URLs
const SNS_ROOT = 'tw2vt-hqaaa-aaaaq-aab6a-cai'

export const links = {
  app: 'https://app.gold-dao.org',
  docs: 'https://docs.gold-dao.org',
  stakingGuide: 'https://docs.gold-dao.org/how-to/stake-the-goldao-tokens',
  proposals: `https://nns.ic0.app/proposals/?u=${SNS_ROOT}`,
  icpDashboard: `https://dashboard.internetcomputer.org/sns/${SNS_ROOT}`,
  x: 'https://x.com/goldaoapp'
}
