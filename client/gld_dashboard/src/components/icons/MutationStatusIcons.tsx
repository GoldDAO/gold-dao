import {
  CheckCircleIcon,
  XCircleIcon,
  PauseCircleIcon,
} from "@heroicons/react/20/solid";

const MutationStatusIcons = ({
  status,
  className,
}: {
  status: "idle" | "pending" | "error" | "success";
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex justify-center h-8 w-8">
        {status === "idle" && <PauseCircleIcon className="" />}
        {status === "pending" && (
          <div className="border-4 h-full w-full border-primary/20 border-t-primary animate-spin rounded-full" />
        )}
        {status === "error" && (
          <XCircleIcon className="text-error text-orange-600" />
        )}
        {status === "success" && (
          <CheckCircleIcon className="text-success text-green-600" />
        )}
      </div>
    </div>
  );
};

export default MutationStatusIcons;
