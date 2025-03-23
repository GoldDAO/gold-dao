import {
  CheckIcon,
  ArrowsRightLeftIcon,
  FireIcon,
  CursorArrowRaysIcon,
} from "@heroicons/react/24/outline";

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
      icon: <CheckIcon className="h-4 w-4" />,
    },
    approve: {
      label: "Approve",
      bgClassName: "bg-amber-100",
      icon: <CursorArrowRaysIcon className="h-4 w-4" />,
    },
    burn: {
      label: "Burn",
      bgClassName: "bg-orange-100",
      icon: <FireIcon className="h-4 w-4" />,
    },
    transfer: {
      label: "Transfer",
      bgClassName: "bg-indigo-100",
      icon: <ArrowsRightLeftIcon className="h-4 w-4" />,
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
