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
  decimals,
  handleOnClick,
}: {
  balance: bigint | undefined;
  fee: bigint | undefined;
  decimals: number | undefined;
  handleOnClick: (amount: string) => void;
}) => {
  const isEnabled =
    balance !== undefined &&
    fee !== undefined &&
    decimals !== undefined &&
    balance >= fee;

  const handleClick = (balance: bigint, fee: bigint, decimals: number) => {
    const maxAmount = Number(balance - fee) / 10 ** decimals;
    handleOnClick(maxAmount.toString());
  };

  if (!isEnabled) {
    return <MaxButtonUI disabled />;
  }

  return (
    <MaxButtonUI handleOnClick={() => handleClick(balance, fee, decimals)} />
  );
};

export default MaxButton;
