import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";

import { SNS_ROOT_CANISTER_ID_IC } from "@constants";

import fetch_one_proposal, { ProposalData } from "../fetch_one_proposal";

export type { ProposalData };

const useFetchOneProposal = ({
  enabled = true,
  placeholderData = keepPreviousData,
  proposalId,
}: Omit<UseQueryOptions<ProposalData>, "queryKey" | "queryFn"> & {
  proposalId: string;
}) => {
  return useQuery({
    queryKey: ["FETCH_ONE_PROPOSAL", proposalId],
    queryFn: async () => {
      try {
        const data = await fetch_one_proposal({
          canisterId: SNS_ROOT_CANISTER_ID_IC,
          proposalId,
        });

        return data;
      } catch (err) {
        console.error(err);
        throw new Error("Fetch one proposal error! Please retry later.");
      }
    },
    placeholderData,
    enabled,
  });
};

export default useFetchOneProposal;
