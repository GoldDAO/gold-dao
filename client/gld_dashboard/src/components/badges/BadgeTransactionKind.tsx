import Icon from "@shared/ui/icons";

export const BadgeTransactionKind = ({
  kind,
  className,
}: {
  kind: "mint" | "approve" | "burn" | "transfer";
  className?: string;
}) => {
  const txKinds = {
    mint: {
      label: "Mint",
      bgClassName: "bg-teal-100",
      icon: <Icon.Mint width={16} height={16} />,
    },
    approve: {
      label: "Approve",
      bgClassName: "bg-amber-100",
      icon: <Icon.Check width={16} height={16} />,
    },
    burn: {
      label: "Burn",
      bgClassName: "bg-orange-100",
      icon: <Icon.Burn width={16} height={16} />,
    },
    transfer: {
      label: "Transfer",
      bgClassName: "bg-indigo-100",
      icon: <Icon.Transfer width={16} height={16} />,
    },
  };
  const k = txKinds[kind];

  return (
    <div className={className ?? ""}>
      <div
        className={`rounded-full font-semibold px-4 py-1 text-sm flex items-center justify-center gap-2 ${k.bgClassName} text-black py-2 px-2`}
      >
        {k.icon}
        <div className="text-xs font-semibold shrink-0 text-black">
          {k.label}
        </div>
      </div>
    </div>
  );
};
