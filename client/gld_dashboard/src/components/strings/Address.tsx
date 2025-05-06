import CopyToClipboard from "@components/buttons/CopyToClipboard";

const Address = ({
  value,
  enableCopyToClipboard = true,
  enableTooltip = true,
  tooltipId = "tooltip",
}: {
  value: string;
  enableCopyToClipboard?: boolean;
  enableTooltip?: boolean;
  tooltipId?: string;
}) => {
  return (
    <div className="max-w-32 flex items-center gap-2">
      <div
        className="truncate"
        {...(enableTooltip && { "data-tooltip-id": tooltipId })}
        {...(enableTooltip && { "data-tooltip-content": value })}
      >
        {value}
      </div>
      {enableCopyToClipboard && <CopyToClipboard value={value} />}
    </div>
  );
};

export default Address;
