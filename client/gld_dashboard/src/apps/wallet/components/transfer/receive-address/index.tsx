import { useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import { QRCodeSVG } from "qrcode.react";
import { useAuth } from "@auth/index";
import { TransferTokenStateAtom } from "@wallet/atoms/TransferTokenAtom";
import Address from "@components/strings/Address";
import { Button } from "@components/index";

const ReceiveAddress = () => {
  const { principalId } = useAuth();
  const setTransferState = useSetAtom(TransferTokenStateAtom);

  const handleClose = () => {
    setTransferState(RESET);
  };

  return (
    <>
      <div className="flex flex-col items-center justify-center gap-4 xl:gap-6 border border-border rounded-lg px-4 py-4 xl:py-8">
        <div className="flex flex-col items-center justify-center gap-1 text-4xl font-semibold">
          <div>Scan to</div>
          <div className="text-primary">receive</div>
        </div>
        <div>
          <QRCodeSVG value={principalId} size={160} />
        </div>
      </div>
      <div className="mt-4 xl:mt-8">
        <div>Principal ID</div>
        <div className="mt-2 bg-surface-secondary border border-border rounded-full px-6 py-3">
          <Address size="lg">{principalId}</Address>
        </div>
      </div>

      <Button
        onClick={handleClose}
        className="w-full mt-4 xl:mt-6 px-6 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
      >
        Close
      </Button>
    </>
  );
};

export default ReceiveAddress;
