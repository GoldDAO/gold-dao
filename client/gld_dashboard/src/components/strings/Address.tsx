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
  size?: "sm" | "md" | "lg" | "auto";
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
      case "auto":
        return "max-w-32 md:max-w-64 lg:max-w-100";
    }
  };

  return (
    <div className={`${getSize(size)} flex items-center gap-2`}>
      <div
        className="truncate"
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
