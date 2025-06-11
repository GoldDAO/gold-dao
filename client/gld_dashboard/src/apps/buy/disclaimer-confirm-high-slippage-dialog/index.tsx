import { Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import { Warning2 } from "iconsax-react";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

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
          <Warning2 size={32} className="text-amber-500" variant="Bold" />
          <div>High slippage</div>
        </div>
      </div>
      <div className="text-center text-content/60 mb-8">
        Slippage is quite high for this purchase.
        <br />
        <div className="inline-block max-w-md mx-auto">
          The current slippage is{" "}
          <span className="text-amber-500 font-semibold">
            <NumberToLocaleString value={slippage} decimals={2} />%
          </span>{" "}
          , which exceeds the maximum recommended slippage of {maxSlippage}%.
        </div>
        <br />
        <div className="mt-2">
          Please confirm or consider purchasing a smaller amount.
        </div>
      </div>
      <div className="flex justify-end gap-2">
        <Button
          className="px-6 py-2 bg-surface text-content rounded-full"
          onClick={handleClose}
        >
          Cancel
        </Button>
        <Button
          className="px-6 py-2 bg-secondary text-white rounded-full"
          onClick={handleConfirm}
        >
          Confirm
        </Button>
      </div>
    </Dialog>
  );
};

export default DisclaimerConfirmHighSlippageDialog;
