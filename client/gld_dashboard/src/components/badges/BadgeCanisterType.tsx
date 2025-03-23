import _capitalize from "lodash/capitalize";

export type CanisterType =
  | "archives"
  | "dapps"
  | "governance"
  | "index"
  | "root"
  | "ledger"
  | "swap";

export const BadgeCanisterType = ({
  type,
  className,
}: {
  type: CanisterType;
  className?: string;
}) => {
  const canisterType = {
    archives: {
      label: "Archive",
      bgCn: "bg-[#ffe2d7]",
    },
    dapps: {
      label: "Dapp",
      bgCn: "bg-[#9b95d5]",
    },
    governance: {
      label: "Governance",
      bgCn: "bg-[#e0e8ff]",
    },
    index: {
      label: "Index",
      bgCn: "bg-[#f6fed8]",
    },
    root: {
      label: "Root",
      bgCn: "bg-[#ede0ff]",
    },
    ledger: {
      label: "Ledger",
      bgCn: "bg-[#d9f2e8]",
    },
    swap: {
      label: "Swap",
      bgCn: "bg-[#feefd8]",
    },
  };

  const ct = canisterType[type] ?? {
    label: _capitalize(type),
    bgCn: "bg-[#95c7d5]",
  };

  return (
    <div className={className ?? ""}>
      <div
        className={`rounded-full px-4 py-1 flex items-center justify-center gap-2 ${ct.bgCn} py-2 px-2`}
      >
        <div className={`text-xs font-semibold shrink-0 text-black`}>
          {ct.label}
        </div>
      </div>
    </div>
  );
};
