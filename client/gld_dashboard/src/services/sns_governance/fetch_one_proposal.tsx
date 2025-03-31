import instance from "./icp_sns_api/instance";
import { Proposal, ProposalData } from "./utils/interfaces";
import { parseProposalData } from "./utils/parseProposalData";

const fetch_one_proposal = async ({
  canisterId,
  proposalId,
}: {} & { canisterId: string; proposalId: string }): Promise<ProposalData> => {
  const { data: proposal } = (await instance.get(
    `/snses/${canisterId}/proposals/${proposalId}`
  )) as { data: Proposal };

  const data = parseProposalData(proposal);

  return data;
};

export type { ProposalData };

export default fetch_one_proposal;
