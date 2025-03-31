import instance from "./icp_sns_api/instance";
import { Proposal, ProposalData } from "./utils/interfaces";
import { parseProposalData } from "./utils/parseProposalData";

interface Proposals {
  data: Proposal[];
  total: number;
  max_proposal_index?: number;
}

const fetch_all_proposals = async ({
  limit = 20,
  offset = 0,
  sort_by = undefined,
  canisterId,
}: {
  limit?: number;
  offset?: number;
  sort_by?: string;
} & { canisterId: string }): Promise<{
  data: ProposalData[];
  total_proposals: number;
  max_proposal_index?: number;
}> => {
  const { data: proposals } = (await instance.get(
    `/snses/${canisterId}/proposals?limit=${limit}&offset=${offset}${
      sort_by ? `&sort_by=${sort_by}` : ""
    }`
  )) as { data: Proposals };

  const data =
    proposals?.data?.map((proposal) => {
      return parseProposalData(proposal);
    }) ?? [];

  return {
    data,
    total_proposals: proposals?.total ?? 0,
    max_proposal_index: proposals?.max_proposal_index,
  };
};

export type { ProposalData };

export default fetch_all_proposals;
