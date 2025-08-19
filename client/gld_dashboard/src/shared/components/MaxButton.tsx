import clsx from "clsx";

const MaxButton = ({
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
      disabled={disabled}
    >
      Max
    </button>
  );
};

export default MaxButton;
