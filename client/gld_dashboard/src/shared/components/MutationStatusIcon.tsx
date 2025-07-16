import Icon from "@shared/ui/icons";

function getSize(size: "sm" | "md" | "lg") {
  switch (size) {
    case "sm":
      return "w-8 h-8";
    case "md":
      return "w-12 h-12";
    case "lg":
      return "w-16 h-16";
    default:
      return "w-8 h-8";
  }
}

const MutationStatusIcon = ({
  status,
  className,
  size = "sm",
}: {
  status: "idle" | "pending" | "error" | "success";
  className?: string;
  size?: "sm" | "md" | "lg";
}) => {
  return (
    <div className={className}>
      <div className={`flex justify-center items-center ${getSize(size)}`}>
        {status === "idle" && <Icon.PauseCircle />}
        {status === "pending" && (
          <div className="border-4 h-full w-full border-gold/20 border-t-gold animate-spin rounded-full" />
        )}
        {status === "error" && <Icon.ErrorCircle className="text-danger" />}
        {status === "success" && (
          <Icon.SuccessCircle className="text-success" />
        )}
      </div>
    </div>
  );
};

export default MutationStatusIcon;
