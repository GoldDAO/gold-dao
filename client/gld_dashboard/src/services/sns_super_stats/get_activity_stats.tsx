import { ActorSubclass } from "@dfinity/agent";
import { ActivitySnapshot } from "./interfaces";
// import { parseSNSCanistersData } from "./utils/parseSNSCanistersData";
// import { SNSCanistersSummaryData } from "./utils/interfaces";

const get_activity_stats = async (
  actor: ActorSubclass,
  options: { timestamp: number }
): Promise<ActivitySnapshot[]> => {
  const results = (await actor.get_activity_stats(
    options.timestamp
  )) as Array<ActivitySnapshot>;
  //   const data = parseSNSCanistersData(results ?? []);
  //   return data;
  return results;
};

// export type { SNSCanistersSummaryData };

export default get_activity_stats;
