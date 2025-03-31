import { ActorSubclass } from "@dfinity/agent";
import { GetSnsCanistersSummaryResponse } from "./interfaces";
import { parseSNSCanistersData } from "./utils/parseSNSCanistersData";
import { SNSCanistersSummaryData } from "./utils/interfaces";

const get_sns_canisters_summary = async ({
  actor,
}: {
  actor: ActorSubclass;
}): Promise<SNSCanistersSummaryData[]> => {
  const results = (await actor.get_sns_canisters_summary({
    update_canister_list: [],
  })) as GetSnsCanistersSummaryResponse;

  const data = parseSNSCanistersData(results ?? []);
  return data;
};

export type { SNSCanistersSummaryData };

export default get_sns_canisters_summary;
