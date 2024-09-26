import { ReactNode } from "react";
import {
  CheckCircleIcon,
  XCircleIcon,
  ArrowPathIcon,
} from "@heroicons/react/24/outline";

const Badge = ({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div
      className={`rounded-full px-4 py-1 text-sm flex items-center justify-center gap-2 ${className}`}
    >
      {children}
    </div>
  );
};

const BadgeStatusTx = ({ status }: { status: string }) => {
  switch (status) {
    case "Success":
      return (
        <Badge className="bg-jade/10">
          <CheckCircleIcon className="h-6 w-6 text-jade" />
          <div className="text-jade font-semibold text-xs">Success</div>
        </Badge>
      );
    case "Failed":
    case "Bid Fail":
      return (
        <Badge className="bg-dark-orange/10">
          <XCircleIcon className="h-6 w-6 text-dark-orange" />
          <div className="text-dark-orange font-semibold text-xs">Failed</div>
        </Badge>
      );
    case "Opening Sale":
    case "Minting":
    case "Swapping NFT":
    case "Burning Fees":
    case "Refunding":
    case "Transfering NFT":
    case "Transfering Fee":
    case "Escrow Request":
    case "Burning":
      return (
        <Badge className="bg-gold/10">
          <ArrowPathIcon className="h-6 w-6 text-gold" />
          <div className="text-gold font-semibold text-xs">{status}...</div>
        </Badge>
      );
    default:
      return null;
  }
};

export default BadgeStatusTx;
