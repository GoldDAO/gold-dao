import Dialog from "@shared/ui/dialog/Dialog";
import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import BtnPrimary from "@shared/ui/button/BtnPrimary";

const DisclaimerConfirmHighSlippageDialog = ({
  open,
  handleConfirm,
  handleClose,
  slippage,
  maxSlippage,
}: {
  open: boolean;
  handleConfirm: () => void;
  handleClose: () => void;
  slippage: number;
  maxSlippage: number;
}) => {
  return (
    <Dialog open={open} handleOnClose={handleClose}>
      <div className="flex justify-center">
        <div className="flex items-center font-semibold text-lg mt-2 mb-4 gap-2">
          <Icon.Warning width={32} className="text-warning" />
          <div>High slippage</div>
        </div>
      </div>
      <div className="text-center text-content/60 mb-8">
        Slippage is quite high for this purchase.
        <br />
        <div className="inline-block max-w-md mx-auto">
          The current slippage is{" "}
          <span className="text-warning font-semibold">
            <NumberToLocaleString value={slippage} />%
          </span>{" "}
          , which exceeds the maximum recommended slippage of {maxSlippage}%.
        </div>
        <br />
        <div className="mt-2">
          Please confirm or consider purchasing a smaller amount.
        </div>
      </div>
      <div className="flex justify-center gap-2">
        <BtnPrimary variant="outlined" onClick={handleClose}>
          Cancel
        </BtnPrimary>
        <BtnPrimary onClick={handleConfirm}>Confirm</BtnPrimary>
      </div>
    </Dialog>
  );
};

export default DisclaimerConfirmHighSlippageDialog;
