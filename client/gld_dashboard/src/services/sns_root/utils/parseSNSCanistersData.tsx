import { Buffer } from "buffer";
import { Principal } from "@dfinity/principal";

import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";
import { GetSnsCanistersSummaryResponse, CanisterSummary } from "../interfaces";
import { SNSCanistersSummaryData } from "./interfaces";

export const parseSNSCanistersData = (sc: GetSnsCanistersSummaryResponse) => {
  const data = Object.entries(sc).flatMap(([key, entries]) =>
    entries.map((canister: CanisterSummary) => {
      const canisterID = Principal.from(canister.canister_id[0]).toText();
      const status = canister?.status[0];
      const settings = status?.settings;

      const cyclesBalance = status?.cycles
        ? (divideBy1e8(status.cycles) / 10e4).toFixed(4)
        : undefined;

      const runningStatus = Object.keys(
        status?.status ?? { "out of cycles": null }
      )[0];

      const type = key;

      const memorySize = status?.memory_size
        ? (Number(status.memory_size) / 1048576).toFixed(2)
        : undefined;

      const freezingThreshold = settings?.freezing_threshold;

      const idleCyclesBurnedPerDay = status?.idle_cycles_burned_per_day;

      const freezingThresholdCycles =
        freezingThreshold && idleCyclesBurnedPerDay
          ? (
              (Number(idleCyclesBurnedPerDay) / (24 * 3600)) *
              (Number(freezingThreshold) / 10e11)
            ).toFixed(4)
          : undefined;

      const moduleHash = status?.module_hash[0]
        ? Buffer.from(status.module_hash[0]).toString("hex")
        : undefined;

      // ? Can have more than one controllers DUSTIN
      const controllers = settings?.controllers[0]
        ? Principal.from(settings.controllers[0]).toText()
        : undefined;

      return {
        canisterID,
        type,
        status: runningStatus,
        cyclesBalance,
        memorySize,
        idleCyclesBurnedPerDay: roundAndFormatLocale({
          number: Number(idleCyclesBurnedPerDay),
        }),
        freezingThresholdCycles,
        moduleHash,
        controllers,
      };
    })
  );
  return data as SNSCanistersSummaryData[];
};
