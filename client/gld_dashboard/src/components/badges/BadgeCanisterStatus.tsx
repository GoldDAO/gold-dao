import _capitalize from "lodash/capitalize";

export type CanisterStatus =
  | "stopped"
  | "stopping"
  | "running"
  | "out of cycles";

export const BadgeCanisterStatus = ({
  status,
  className,
}: {
  status: CanisterStatus;
  className?: string;
}) => {
  const canisterStatus = {
    stopped: {
      label: "Stopped",
      bgCn: "bg-[#ffe2d7]",
    },
    stopping: {
      label: "Stopping",
      bgCn: "bg-[#f6fed8]",
    },
    running: {
      label: "Running",
      bgCn: "bg-[#d9f2e8]",
    },
    "out of cycles": {
      label: "Out of cycles",
      bgCn: "bg-[#ff9898]",
    },
  };

  const cs = canisterStatus[status] ?? {
    label: _capitalize(status),
    bgCn: "bg-[#c7d0d3]",
  };

  return (
    <div className={className ?? ""}>
      <div
        className={`rounded-full px-4 py-1 flex items-center justify-center gap-2 ${cs.bgCn} py-2 px-2`}
      >
        <div className={`text-xs font-semibold shrink-0 text-black`}>
          {cs.label}
        </div>
      </div>
    </div>
  );
};
