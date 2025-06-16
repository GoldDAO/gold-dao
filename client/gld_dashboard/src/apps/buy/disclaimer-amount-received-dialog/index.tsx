import Dialog from "@components/dialogs/Dialog";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const DisclaimerAmountReceivedDialog = ({
  open,
  handleClose,
}: {
  open: boolean;
  handleClose: () => void;
}) => {
  return (
    <Dialog open={open} handleOnClose={handleClose}>
      <div className="text-center">
        <div className="font-semibold text-lg mb-4">Receive amount</div>
        <div className="text-content/60 mb-8">
          The exact amount of GLDT received will vary due to market fluctuations
          and slippage.
        </div>
        <div className="flex justify-end">
          <BtnPrimary onClick={handleClose}>Close</BtnPrimary>
        </div>
      </div>
    </Dialog>
  );
};

export default DisclaimerAmountReceivedDialog;
