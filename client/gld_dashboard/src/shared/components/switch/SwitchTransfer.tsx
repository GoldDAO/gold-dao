type SwitchValue = "send" | "receive";

const BtnSwitch = ({
  label,
  value,
  selected,
  handleOnClick,
}: {
  label: string;
  value: SwitchValue;
  selected: boolean;
  handleOnClick: (value: SwitchValue) => void;
}) => (
  <button
    onClick={() => handleOnClick(value)}
    className={`cursor-pointer hover:opacity-80 rounded-full px-6 py-3 font-medium ${
      selected
        ? "bg-primary text-white"
        : "bg-surface-secondary text-content/60"
    }`}
  >
    {label}
  </button>
);

const SwitchTransfer = ({
  value,
  handleChange,
  className,
}: {
  value: SwitchValue;
  handleChange: (value: SwitchValue) => void;
  className?: string;
}) => (
  <div className={className}>
    <div className="inline-flex justify-center items-center bg-surface-secondary rounded-full">
      <BtnSwitch
        label="Send"
        value="send"
        selected={value === "send"}
        handleOnClick={handleChange}
      />
      <BtnSwitch
        label="Receive"
        value="receive"
        selected={value === "receive"}
        handleOnClick={handleChange}
      />
    </div>
  </div>
);

export default SwitchTransfer;
