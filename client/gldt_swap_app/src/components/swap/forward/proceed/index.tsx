import { useQueryClient } from "@tanstack/react-query";
import { ArrowDownIcon } from "@heroicons/react/20/solid";

import { useNft, useForwardSwapProceed } from "@context/index";

import { Dialog, Button } from "@components/ui";
import TransactionDetails from "../TransactionDetails";

import Pending from "./Pending";
import Success from "./Success";
import Error from "./Error";

const Proceed = () => {
  const {
    getSelectedTotalGLDT,
    getSelectedTotalGram,
    // getCollectionSelectedNFTs,
    resetState: resetSwapState,
  } = useNft();
  const totalGram = getSelectedTotalGram();
  const totalGLDT = getSelectedTotalGLDT();
  // const selectedNfts = getCollectionSelectedNFTs();
  const queryClient = useQueryClient();
  const {
    state: forwardSwapProceedState,
    handleShow,
    handleClose,
    forwardSwap,
    setCanCloseDialog,
  } = useForwardSwapProceed();
  const { show, canCloseDialog } = forwardSwapProceedState;
  const {
    mutate: mutateSwapGLDNFT,
    isSuccess,
    isError,
    isPending,
    isIdle,
  } = forwardSwap;

  const handleOnClick = () => {
    setCanCloseDialog(false);
    mutateSwapGLDNFT(undefined, {
      onSuccess: () => {
        setCanCloseDialog(true);
        queryClient.invalidateQueries({
          queryKey: ["USER_GET_GLD_NFT_1G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["USER_GET_GLD_NFT_10G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["USER_GET_GLD_NFT_100G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["USER_GET_GLD_NFT_1000G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["USER_FETCH_ACTIVE_SWAPS"],
        });
        resetSwapState();
      },
      onError: (err) => {
        console.log(err);
        setCanCloseDialog(true);
      },
    });
  };

  return (
    <>
      <Button
        onClick={handleShow}
        className={`rounded-xl w-full py-3`}
        disabled={!totalGLDT}
      >
        Proceed to Swap
      </Button>
      <Dialog
        show={show}
        handleClose={handleClose}
        enableClose={canCloseDialog}
      >
        <div className="px-6 pt-6 pb-12">
          {isIdle && (
            <>
              <div className="flex flex-col items-center gap-6 border border-border bg-surface-2 p-6 rounded-xl">
                <div className="font-semibold">{totalGram}g of gold</div>

                <div className="w-full flex justify-center items-center py-4">
                  <div className="relative w-full">
                    <div className="border-t border-border w-full"></div>
                    <div className="absolute inset-x-0 top-0 flex justify-center transform -translate-y-1/2">
                      <button className="bg-content text-background rounded-full p-2 cursor-default">
                        <ArrowDownIcon
                          height={24}
                          width={24}
                          className="text-gold"
                        />
                      </button>
                    </div>
                  </div>
                </div>
                <div className="font-semibold">{totalGLDT} GLDT</div>
              </div>

              <TransactionDetails className="w-full  mt-8" />

              <div className="text-center mt-8">
                <Button onClick={handleOnClick}>Convert</Button>
              </div>
            </>
          )}
          {isPending && <Pending />}
          {isSuccess && <Success />}
          {isError && <Error />}
        </div>
      </Dialog>
    </>
  );
};

export default Proceed;
