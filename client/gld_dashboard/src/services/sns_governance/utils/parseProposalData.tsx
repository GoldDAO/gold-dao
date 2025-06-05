import { divideBy1e8 } from "@shared/utils/numbers";
import { getDateUTC } from "@shared/utils/dates";

import { ProposalData, Proposal } from "./interfaces";

export const parseProposalData = (proposal: Proposal) => {
  const data: ProposalData = {
    id: proposal.id,
    proposer: proposal.proposer,
    title: proposal.proposal_title,
    url: proposal.proposal_url,
    created_at: getDateUTC(proposal.proposal_creation_timestamp_seconds, {
      fromSeconds: true,
    }),
    timeRemaining:
      proposal.wait_for_quiet_state_current_deadline_timestamp_seconds,
    type: proposal.proposal_action_type,
    summary: proposal.summary,
    status: proposal.status.toLowerCase(),
    payload: proposal.payload_text_rendering,
    latestTally: {
      yes: divideBy1e8(proposal.latest_tally.yes),
      no: divideBy1e8(proposal.latest_tally.no),
      total: divideBy1e8(proposal.latest_tally.total),
    },
    votes: divideBy1e8(proposal.votes),
    riskedOGY: divideBy1e8(proposal.reject_cost_e8s),
  };

  return data;
};
