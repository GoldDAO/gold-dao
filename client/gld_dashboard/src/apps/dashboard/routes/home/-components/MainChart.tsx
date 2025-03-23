import clsx from "clsx";
// import { useState } from "react";
import { useLocalStorage } from "usehooks-ts";

import { SNS_SUPER_STATS_CANISTER_ID } from "@constants";
import useFetchHolders from "@services/sns_super_stats/hooks/useFetchHolders";
import { useAuth } from "@auth/index";

type PERIOD = "1D" | "1W" | "1M" | "3M" | "1Y" | "All";

const ChartPeriodSelector = ({
  className,
  selectedPeriod = "1M",
  periods = ["1D", "1W", "1M", "3M", "1Y", "All"],
  handleOnChangePeriod,
}: {
  className?: string;
  selectedPeriod?: PERIOD;
  periods?: PERIOD[];
  handleOnChangePeriod: (period: PERIOD) => void;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center justify-between">
        {periods.map((period) => (
          <div
            key={period}
            className={clsx(
              "flex-1 px-1 py-3 bg-surface-secondary cursor-pointer",
              "text-center text-sm font-semibold",
              "border-r border-border last:border-r-0",
              `${period === selectedPeriod ? "text-accent" : "text-content/60"}`,
              "hover:text-accent"
            )}
            onClick={() => handleOnChangePeriod(period)}
          >
            {period}
          </div>
        ))}
      </div>
    </div>
  );
};

// const TreasuryChart = ({ className }: { className?: string }) => {
//   const [period, setPeriod] = useLocalStorage<PERIOD>(
//     "chart-treasury-period",
//     "1M"
//   );
//   return (
//     <div className={className}>
//       <div className="mb-6">Treasury Chart {period}</div>
//       <ChartPeriodSelector
//         handleOnChangePeriod={setPeriod}
//         selectedPeriod={period}
//       />
//     </div>
//   );
// };

const HoldersChart = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent } = useAuth();

  const [period, setPeriod] = useLocalStorage<PERIOD>(
    "chart-holders-period",
    "1M"
  );

  useFetchHolders(SNS_SUPER_STATS_CANISTER_ID, unauthenticatedAgent, {
    enabled: !!unauthenticatedAgent,
  });

  return (
    <div className={className}>
      <div className="mb-6">Holders Chart {period}</div>
      <ChartPeriodSelector
        handleOnChangePeriod={setPeriod}
        selectedPeriod={period}
      />
    </div>
  );
};

const MainChart = ({ className }: { className?: string }) => {
  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary">
        <div className="p-6">
          <HoldersChart />
        </div>
      </div>
    </div>
  );
};

export default MainChart;
