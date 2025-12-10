import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import Dialog from "@shared/ui/dialog/Dialog";
import { useAuth } from "@auth/index";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import {
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
  TotalGramSelectedAtom,
  TotalGLDTSelectedAtom,
  CollectionSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import Icon from "@shared/ui/icons";
// import { SelectNFTStateReducerAtom } from "@atoms/NFTState";

const Confirm = () => {
  const { principalId } = useAuth();
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const totalGramsSelected = useAtomValue(TotalGramSelectedAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const collectionsSelected = useAtomValue(CollectionSelectedAtom);

  const handleClose = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchTransferNFTState({ type: "RESET" });
  };
  const { send_receive_address } = transferNFTState;

  const renderConfirm = () => {
    if (send_receive_address === "") {
      return (
        <div className="flex justify-center items-center px-4 py-16 xl:py-32">
          Loading...
        </div>
      );
    } else {
      return (
        <>
          <div className="border border-border rounded-md xl:rounded-lg p-4">
            <div className="mb-2">Sending Account</div>
            <div className="text-content/60 text-sm">{principalId}</div>

            <div className="my-4 xl:my-6 text-content/20 border-b border-dashed" />

            <div>Receiving Account</div>
            <div className="text-content/60 text-sm mt-2">
              {send_receive_address}
            </div>

            <div className="my-4 xl:my-6 text-content/20 border-b border-dashed" />

            <div className="mb-2">Total</div>
            <div className={clsx("flex flex-col gap-2 text-sm")}>
              <div className="flex flex-col xl:flex-row xl:justify-between gap-2">
                <div className="text-content/60">Total NFTs</div>
                <div className="font-semibold">{totalNFTSelected} NFT(s)</div>
              </div>

              <div className="flex flex-col xl:flex-row xl:justify-between gap-2">
                <div className="text-content/60">Total Weight</div>
                <div className="font-semibold">{totalGramsSelected} g</div>
              </div>

              <div className="flex flex-col xl:flex-row xl:justify-between gap-2">
                <div className="text-content/60">Total GLDT Value</div>
                <div className="font-semibold">
                  {totalGLDTSelected.toLocaleString()} GLDT
                </div>
              </div>

              {collectionsSelected.length > 0 && (
                <>
                  <div className="my-2 border-t border-border" />
                  <div className="text-content/60 mb-2">By Collection:</div>
                  {collectionsSelected.map((collection) => (
                    <div
                      key={collection.name}
                      className="flex items-center justify-between gap-2 text-sm"
                    >
                      <div className="flex items-center gap-2">
                        <img
                          className="flex-none h-6"
                          src={`/gold-bars/${collection.name}.png`}
                          alt={collection.label}
                        />
                        <span className="text-content/60">
                          {collection.label}
                        </span>
                      </div>
                      <div className="font-semibold">
                        {collection.total_count_selected} NFT(s) -{" "}
                        {collection.total_grams_selected} g
                      </div>
                    </div>
                  ))}
                </>
              )}
            </div>
          </div>
          <div className="mt-8">
            <BtnPrimary
              onClick={() =>
                dispatchTransferNFTState({ type: "OPEN_SEND_DIALOG_DETAILS" })
              }
              className="w-full"
            >
              Confirm Transfer
            </BtnPrimary>
          </div>
        </>
      );
    }
  };

  return (
    <Dialog
      open={transferNFTState.is_open_send_dialog_confirm}
      handleOnClose={handleClose}
      size="xl"
      title={
        <div
          className={clsx(
            "p-1 rounded-full cursor-pointer",
            "hover:bg-primary hover:text-white"
          )}
          onClick={() =>
            dispatchTransferNFTState({ type: "CANCEL_SEND_CONFIRM" })
          }
        >
          <Icon.Chevron width={18} height={18} className="rotate-90" />
        </div>
      }
    >
      {renderConfirm()}
    </Dialog>
  );
};

export default Confirm;
