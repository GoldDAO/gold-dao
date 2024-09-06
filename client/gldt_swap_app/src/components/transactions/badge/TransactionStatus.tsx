import { ReactNode } from "react";
import { CheckCircleIcon, XCircleIcon } from "@heroicons/react/24/outline";

const Badge = ({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div
      className={`rounded-full font-semibold w-[120px] py-1 text-sm flex items-center justify-center gap-2 ${className}`}
    >
      {children}
    </div>
  );
};

const BadgeStatusTx = ({ status }: { status: string }) => {
  switch (status) {
    case "Success":
      return (
        <Badge className="bg-emerald-500/10">
          <CheckCircleIcon className="h-6 w-6 text-emerald-500" />
          <div className="text-emerald-500">Success</div>
        </Badge>
      );
    case "Failed":
      return (
        <Badge className="bg-red-500/10">
          <XCircleIcon className="h-6 w-6 text-red-500" />
          <div className="text-red-500">Failed</div>
        </Badge>
      );
    default:
      null;
  }
};

export default BadgeStatusTx;
