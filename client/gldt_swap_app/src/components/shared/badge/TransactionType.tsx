import { ReactNode } from "react";
import {
  CheckIcon,
  ArrowsRightLeftIcon,
  FireIcon,
  CursorArrowRaysIcon,
} from "@heroicons/react/24/outline";

export const BadgeTransactionType = ({
  type,
  className,
}: {
  type: string;
  className?: string;
}) => {
  const TYPES: {
    [key: string]: { name: string; bgColorCn: string; icon: ReactNode };
  } = {
    transfer: {
      name: "Transfer",
      // bgColorCn: "bg-indigo-100",
      bgColorCn: "bg-gold/20",
      icon: <ArrowsRightLeftIcon className="h-4 w-4" />,
    },
    mint: {
      name: "Mint",
      // bgColorCn: "bg-teal-100",
      bgColorCn: "bg-gold/20",
      icon: <CursorArrowRaysIcon className="h-4 w-4" />,
    },
    approve: {
      name: "Approve",
      // bgColorCn: "bg-amber-100",
      bgColorCn: "bg-gold/20",
      icon: <CheckIcon className="h-4 w-4" />,
    },
    burn: {
      name: "Burn",
      // bgColorCn: "bg-orange-100",
      bgColorCn: "bg-gold/20",
      icon: <FireIcon className="h-4 w-4" />,
    },
  };

  return (
    <div className={className}>
      {TYPES[type] && (
        <div
          className={`rounded-full font-semibold px-4 py-1 text-sm text-gold flex items-center justify-center gap-2 ${TYPES[type].bgColorCn}`}
        >
          {TYPES[type].icon}
          <div>{TYPES[type].name}</div>
        </div>
      )}
    </div>
  );
};
