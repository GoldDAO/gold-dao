export const BadgeProposalStatus = ({
  status,
  className,
}: {
  status: "open" | "executed";
  className?: string;
}) => {
  const proposalStatus = {
    open: {
      label: "Open",
      bgCn: "bg-[#f6f28a]",
      textCn: "text-sky",
    },
    executed: {
      label: "Executed",
      bgCn: "bg-[#d9f2e8]",
      textCn: "text-jade",
    },
    rejected: {
      label: "Rejected",
      bgCn: "bg-[#f2dcd9]",
      textCn: "text-jade",
    },
  };
  const p = proposalStatus[status];

  return (
    <div className={className ?? ""}>
      <div
        className={`rounded-full px-4 py-1 flex items-center justify-center gap-2 ${p.bgCn} py-2 px-2`}
      >
        <div className={`text-xs font-semibold shrink-0 text-black`}>
          {p.label}
        </div>
      </div>
    </div>
  );
};
