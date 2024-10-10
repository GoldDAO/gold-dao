import { FieldValues } from "react-hook-form";

import { Button, LoaderSpin } from "@components/ui";
import {
  Count as SelectNFTCount,
  Title as SelectNFTTitle,
} from "@components/shared/nft-select";
import Loading from "@components/shared/user-nft/Loading";
import Empty from "@components/shared/user-nft/Empty";
import Error from "@components/shared/user-nft/Error";

import { useNft } from "@context/nft";
import { useTransferProceedNft } from "@context/transfer/proceed-nft";
import { useGetUserGLDNFT } from "@hooks/gld_nft";

import FieldTo from "./form/To";
import Balance from "../shared/balance/Balance";

import ConfirmDialog from "./confirm-dialog";
import BuyOGYDisclaimer from "./buy-ogy-disclaimer/index";

const TransferNft = () => {
  const { getCountNfts, selectNft, unselectNft, state: nftState } = useNft();
  const { form, handleShowDialogConfirm, handleSubmitForm, balanceOGY, state } =
    useTransferProceedNft();
  const { totalTransferFee, canTransfer, isInsufficientOGYFunds } = state;
  const countUserNft = getCountNfts();
  const {
    isLoading: isLoadingUserNft,
    isSuccess: isSuccessUserNft,
    isError: isErrorUserNft,
    error: errorUserNft,
  } = useGetUserGLDNFT();

  const {
    handleSubmit,
    // control,
    formState: { isValid },
  } = form;

  const handleOnChangeCount = (collectionIndex: number, type: string) => {
    if (type === "-") {
      unselectNft(collectionIndex);
    } else if (type === "+") {
      selectNft(collectionIndex);
    }
  };

  const onSubmit = (data: FieldValues) => {
    handleSubmitForm(data as { to: string });
    handleShowDialogConfirm();
  };

  return (
    <>
      {balanceOGY.isLoading && (
        <div className="flex justify-center py-16">
          <LoaderSpin />
        </div>
      )}
      {balanceOGY.isSuccess && (
        <>
          <form onSubmit={handleSubmit(onSubmit)}>
            <div className="mt-4 p-4 border border-border rounded-xl">
              <FieldTo />
            </div>
            <div className="mt-4 p-4 border border-border rounded-xl text-center sm:text-left">
              <div className="text-gold text-sm font-semibold mb-2">Amount</div>
              {isLoadingUserNft && <Loading />}
              {!isLoadingUserNft && nftState.isEmpty && <Empty />}
              {isErrorUserNft && <Error error={errorUserNft} />}
              {isSuccessUserNft &&
                nftState.nfts.map((d, index) => {
                  return (
                    !d.isEmpty && (
                      <div
                        key={d.name}
                        className="flex justify-center items-center border border-border bg-surface-2 py-3 px-1 sm:px-4 rounded-xl mb-2 last:mb-0 sm:gap-12 gap-4"
                      >
                        <div className="flex items-center justify-between w-[260px]">
                          <SelectNFTTitle collectionName={d.name} />
                          <SelectNFTCount
                            collectionIndex={index}
                            count={countUserNft[index].selected}
                            handleOnChangeCount={handleOnChangeCount}
                          />
                          <div className="text-content/60 text-sm">
                            {countUserNft[index].selected} /{" "}
                            {countUserNft[index].total}
                          </div>
                        </div>
                      </div>
                    )
                  );
                })}
            </div>
            <div className="flex flex-col sm:flex-row justify-between items-center mt-8 mx-2">
              <div className="inline-flex justify-start items-center text-content/60 text-sm rounded-lg mb-2 sm:mb-0">
                <div>Fee: </div>
                <div className="flex items-center">
                  <img
                    className="mx-2 h-4 w-4"
                    src="/ogy_logo.svg"
                    alt="OGY Logo"
                  />
                  <span>{totalTransferFee.string} OGY</span>
                </div>
              </div>
              {/* <div className="inline-flex justify-start items-center text-content/60 text-sm rounded-lg">
                <div>Amount received: </div>
                <div className="flex items-center">
                  <img
                    className="mx-2 h-4 w-4"
                    src={`/${ledger.toLocaleLowerCase()}_logo.svg`}
                    alt={`${ledger} Logo`}
                  />
                  <Amount />
                </div>
              </div> */}
            </div>
            {isInsufficientOGYFunds && <BuyOGYDisclaimer className="mt-6" />}
            <Button
              type="submit"
              disabled={!isValid || !canTransfer}
              className="mt-8 w-full py-3 rounded-lg"
            >
              Transfer
            </Button>
            <div className="flex justify-center mt-6">
              <Balance ledger="OGY" balance={balanceOGY.data.number} />
            </div>
          </form>
          <ConfirmDialog />
        </>
      )}
    </>
  );
};

export default TransferNft;
