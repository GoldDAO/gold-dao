import clsx from "clsx";

const MaxButtonUI = ({
  handleOnClick,
  disabled = false,
}: {
  handleOnClick?: () => void;
  disabled?: boolean;
}) => {
  return (
    <button
      onClick={handleOnClick}
      type="button"
      className={clsx(
        "rounded-md py-1 px-2 bg-surface-primary text-content/60 border border-border text-xs cursor-pointer",
        {
          "disabled:cursor-not-allowed": disabled,
        }
      )}
      data-tooltip-id="tooltip"
      data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
      disabled={disabled}
    >
      Max
    </button>
  );
};

const MaxButton = ({
  balance,
  fee,
  handleOnClick,
}: {
  balance: number | undefined;
  fee: number | undefined;
  handleOnClick: (amount: string) => void;
}) => {
  const isEnabled = balance !== undefined && fee !== undefined && balance > fee;

  const handleClick = (balance: number, fee: number) => {
    const maxAmount = balance - fee;
    handleOnClick(maxAmount.toString());
  };

  if (!isEnabled) {
    return <MaxButtonUI disabled />;
  }

  return <MaxButtonUI handleOnClick={() => handleClick(balance, fee)} />;
};

export default MaxButton;
