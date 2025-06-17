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
          <div className="border-4 h-full w-full border-gold/20 border-t-gold animate-spin rounded-full" />
        )}
        {status === "error" && <XCircleIcon className="text-danger" />}
        {status === "success" && <CheckCircleIcon className="text-success" />}
      </div>
    </div>
  );
};

export default MutationStatusIcons;
