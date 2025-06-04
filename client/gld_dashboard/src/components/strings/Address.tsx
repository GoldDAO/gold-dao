import CopyToClipboard from "@components/buttons/CopyToClipboard";

const Address = ({
  enableCopyToClipboard = true,
  enableTooltip = true,
  tooltipId = "tooltip",
  size = "sm",
  children,
}: {
  enableCopyToClipboard?: boolean;
  enableTooltip?: boolean;
  tooltipId?: string;
  size?: "sm" | "md" | "lg" | "full";
  children: string;
}) => {
  const getSize = (size?: string) => {
    switch (size) {
      case "sm":
        return "max-w-32";
      case "md":
        return "max-w-64";
      case "lg":
        return "max-w-100";
      case "full":
        return "max-w-full";
    }
  };

  return (
    <div className={`${getSize(size)} flex items-center gap-2`}>
      <div
        className={`${size !== "full" ? "truncate" : ""}`}
        {...(enableTooltip && {
          "data-tooltip-id": tooltipId,
          "data-tooltip-content": children,
        })}
      >
        {children}
      </div>
      {enableCopyToClipboard && <CopyToClipboard value={children} />}
    </div>
  );
};

export default Address;
