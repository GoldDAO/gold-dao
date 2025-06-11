import { Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";

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
          <Button
            className="px-6 py-2 bg-secondary text-white rounded-full"
            onClick={handleClose}
          >
            Close
          </Button>
        </div>
      </div>
    </Dialog>
  );
};

export default DisclaimerAmountReceivedDialog;
