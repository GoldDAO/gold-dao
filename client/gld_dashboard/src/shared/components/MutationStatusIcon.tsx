import Icon from "@shared/ui/icons";

const MutationStatusIcon = ({
  status,
  className,
}: {
  status: "idle" | "pending" | "error" | "success";
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex justify-center h-8 w-8">
        {status === "idle" && <Icon.PauseCircle width={24} />}
        {status === "pending" && (
          <div className="border-4 h-full w-full border-gold/20 border-t-gold animate-spin rounded-full" />
        )}
        {status === "error" && (
          <Icon.ErrorCircle width={24} className="text-danger" />
        )}
        {status === "success" && (
          <Icon.SuccessCircle width={24} className="text-success" />
        )}
      </div>
    </div>
  );
};

export default MutationStatusIcon;
