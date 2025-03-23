import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { SortingState } from "@tanstack/react-table";

import { SNS_ROOT_CANISTER_ID_IC } from "@constants";

import fetch_all_proposals, { ProposalData } from "../fetch_all_proposals";

export type { ProposalData };

export interface Proposals {
  data: ProposalData[];
  total_proposals: number;
  max_proposal_index?: number;
}

const useFetchAllProposals = ({
  enabled = true,
  placeholderData = keepPreviousData,
  limit,
  offset,
  sorting,
}: Omit<UseQueryOptions<Proposals>, "queryKey" | "queryFn"> & {
  limit?: number;
  offset?: number;
  sorting?: SortingState;
} = {}) => {
  return useQuery({
    queryKey: ["FETCH_ALL_PROPOSALS", limit, offset, sorting],
    queryFn: async () => {
      let sort_by = undefined;
      if (sorting && sorting[0]) {
        const { id, desc } = sorting[0];
        sort_by = desc ? `-${id}` : id;
      }
      try {
        const data = await fetch_all_proposals({
          limit,
          offset,
          sort_by: sort_by ?? "",
          canisterId: SNS_ROOT_CANISTER_ID_IC,
        });

        return data;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch all proposals error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchAllProposals;
