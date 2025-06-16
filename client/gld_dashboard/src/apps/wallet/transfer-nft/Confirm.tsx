import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";
// import { SelectNFTStateReducerAtom } from "@atoms/NFTState";

const Confirm = () => {
  const { principalId } = useAuth();
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  //   const [selectNFTState, dispatchSelectNFTState] = useAtom(
  //     SelectNFTStateReducerAtom
  //   );

  const { send_receive_address } = transferNFTState;

  if (send_receive_address === "") {
    return (
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
        Loading...
      </div>
    );
  }

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

        {/* <div className={clsx("flex flex-col gap-1 xl:gap-2")}>
          <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-sm">
            <div className="text-content/60">
              Amount deducted (including fee)
            </div>
            <E8sToLocaleString
              value={amount + fee}
              decimals={decimals}
            />{" "}
            {token.name}
          </div>

          <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-content/60 text-sm">
            <div>Fee</div>
            <E8sToLocaleString value={fee} decimals={decimals} />{" "}
            {token.name}
          </div>

          <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-sm">
            <div className="text-content/60">Amount received on new wallet</div>
            <E8sToLocaleString value={amount} decimals={decimals} />{" "}
            {token.name}
          </div>
        </div> */}
      </div>
      <div className="mt-8">
        <BtnPrimary
          onClick={() => dispatchTransferNFTState({ type: "SEND_CONFIRM" })}
          className="w-full"
        >
          Confirm Transfer
        </BtnPrimary>
      </div>
    </>
  );
};

export default Confirm;
