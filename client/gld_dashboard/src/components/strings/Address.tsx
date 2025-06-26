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
  size?: "xs" | "sm" | "md" | "lg" | "auto";
  children: string;
}) => {
  const getSize = (size?: string) => {
    switch (size) {
      case "xs":
        return "max-w-24";
      case "sm":
        return "max-w-32";
      case "md":
        return "max-w-64";
      case "lg":
        return "max-w-100";
      case "auto":
        return "max-w-32 md:max-w-64 lg:max-w-100";
      default:
        return "max-w-32";
    }
  };

  // Inline mapping for start/end chars
  const sizeMap = {
    xs: { start: 2, end: 2 },
    sm: { start: 4, end: 4 },
    md: { start: 8, end: 8 },
    lg: { start: 12, end: 12 },
    auto: { start: 8, end: 8 },
  };
  const { start, end } = sizeMap[size ?? "sm"];
  const startStr = children.slice(0, start);
  const endStr = children.slice(-end);

  return (
    <div className={`${getSize(size)} flex items-center gap-2`}>
      <div
        className="flex items-center overflow-hidden"
        {...(enableTooltip && {
          "data-tooltip-id": tooltipId,
          "data-tooltip-content": children,
          title: children,
        })}
      >
        <span className="truncate">{startStr}</span>
        <span className="flex-shrink-0">…</span>
        <span className="truncate text-right">{endStr}</span>
      </div>
      {enableCopyToClipboard && <CopyToClipboard value={children} />}
    </div>
  );
};

export default Address;
