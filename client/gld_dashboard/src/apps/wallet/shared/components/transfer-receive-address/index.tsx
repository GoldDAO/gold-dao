import { useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import { QRCodeSVG } from "qrcode.react";
import { useAuth } from "@auth/index";
import { TransferTokenStateAtom } from "@wallet/shared/atoms/TransferTokenAtom";
import Address from "@components/strings/Address";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

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
          <div className="text-gold font-semibold">receive</div>
        </div>
        <div className="p-3 bg-white rounded-xl">
          <QRCodeSVG value={principalId} size={160} />
        </div>
      </div>
      <div className="mt-4 xl:mt-8">
        <div>Principal ID</div>
        <div className="mt-2 bg-surface-secondary border border-border rounded-full px-6 py-3">
          <Address size="lg">{principalId}</Address>
        </div>
      </div>

      <BtnPrimary onClick={handleClose} className="w-full mt-4 xl:mt-6">
        Close
      </BtnPrimary>
    </>
  );
};

export default ReceiveAddress;
