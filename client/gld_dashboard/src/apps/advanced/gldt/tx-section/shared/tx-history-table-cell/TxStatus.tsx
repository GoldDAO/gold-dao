import { ReactNode } from "react";
import Icon from "@shared/ui/icons";

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

const TxStatus = ({ status }: { status: string }) => {
  switch (status) {
    case "Complete":
    case "Minted":
    case "Burned":
    case "Nft transferred":
    case "Nft transferred from":
    case "Reimbursed":
      return (
        <Badge className="bg-success/10">
          <Icon.SuccessCircle width={18} height={18} className="text-success" />
          <div className="text-success font-semibold text-xs">{status}</div>
        </Badge>
      );
    case "Failed":
    case "Mint failed":
    case "Burn failed":
    case "Nft transfer failed":
    case "Nft transfer from failed":
    case "Reimburse failed":
    case "Reimbursed failed":
      return (
        <Badge className="bg-danger/10">
          <Icon.ErrorCircle width={18} height={18} className="text-danger" />
          <div className="text-danger font-semibold text-xs">{status}</div>
        </Badge>
      );
    case "Init":
      return (
        <Badge className="bg-gold/10">
          <Icon.Pending width={18} height={18} className="text-gold" />
          <div className="text-gold font-semibold text-xs">{status}...</div>
        </Badge>
      );
    default:
      return null;
  }
};

export default TxStatus;
