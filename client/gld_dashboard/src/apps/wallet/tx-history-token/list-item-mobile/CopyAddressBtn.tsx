import { useState } from "react";
import Dialog from "@shared/ui/dialog/Dialog";
import Icon from "@shared/ui/icons";
import { useCopyToClipboard } from "@shared/hooks/useCopyToClipboard";
import BtnPrimary from "@shared/ui/button/BtnPrimary";

const CopyAddressBtn = ({ from, to }: { from: string; to: string }) => {
  const { copyToClipboard } = useCopyToClipboard();
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <button
        data-tooltip-id="copy-address-tooltip"
        onClick={() => setIsOpen(true)}
      >
        <Icon.Copy width={16} height={16} className="text-content/60" />
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
                <div>{from}</div>
                <div onClick={() => setIsOpen(false)}>
                  <button onClick={() => copyToClipboard(from)}>
                    <Icon.Copy width={16} height={16} />
                  </button>
                </div>
              </div>
            </div>
            <div className="border border-border rounded-xl">
              <div className="border-b border-border bg-surface-secondary rounded-t-xl">
                <div className="p-2 text-base">To</div>
              </div>
              <div className="flex justify-between items-center gap-4 p-2 break-all whitespace-pre-line">
                <div>{to}</div>
                <div onClick={() => setIsOpen(false)}>
                  <button onClick={() => copyToClipboard(to)}>
                    <Icon.Copy width={16} height={16} />
                  </button>
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
