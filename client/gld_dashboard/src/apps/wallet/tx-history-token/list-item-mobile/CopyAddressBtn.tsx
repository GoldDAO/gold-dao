import { useState } from "react";
import { Copy } from "iconsax-react";
import Dialog from "@components/dialogs/Dialog";
import CopyToClipboard from "@components/buttons/CopyToClipboard";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const CopyAddressBtn = ({
  from,
  to,
}: {
  from: string | undefined;
  to: string | undefined;
}) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <button
        data-tooltip-id="copy-address-tooltip"
        onClick={() => setIsOpen(true)}
      >
        <Copy size={16} className="text-content/60" />
      </button>
      <Dialog
        open={isOpen}
        handleOnClose={() => setIsOpen(false)}
        closeIcon={false}
      >
        <div className="py-4">
          <div className="flex flex-col gap-2">
            <div className="border border-border rounded-xl">
              <div className="border-b border-border bg-surface-secondary rounded-t-xl">
                <div className="p-2 text-base">From</div>
              </div>
              <div className="flex justify-between items-center gap-4 p-2 break-all whitespace-pre-line">
                <div>{from ?? "N/A"}</div>
                <div onClick={() => setIsOpen(false)}>
                  <CopyToClipboard value={from} />
                </div>
              </div>
            </div>
            <div className="border border-border rounded-xl">
              <div className="border-b border-border bg-surface-secondary rounded-t-xl">
                <div className="p-2 text-base">To</div>
              </div>
              <div className="flex justify-between items-center gap-4 p-2 break-all whitespace-pre-line">
                <div>{to ?? "N/A"}</div>
                <div onClick={() => setIsOpen(false)}>
                  <CopyToClipboard value={to} />
                </div>
              </div>
            </div>
          </div>
          <BtnPrimary
            className="w-full mt-6"
            shape="round"
            onClick={() => setIsOpen(false)}
          >
            Close
          </BtnPrimary>
        </div>
      </Dialog>
    </>
  );
};

export default CopyAddressBtn;
