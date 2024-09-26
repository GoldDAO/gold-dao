// import { useEffect, useRef, useState } from "react";
import { useAuth } from "@context/auth";
import { useNft } from "@context/index";
import { useGetAvailableGLDNFT } from "@hooks/gld_nft";

import {
  Count as SelectNFTCount,
  Title as SelectNFTTitle,
} from "@components/shared/select-nfts";

import Loading from "./Loading";
import Empty from "./Empty";
import Error from "./Error";
import { useLedgerUserBalance } from "@hooks/ledger";

const ReverseSwapTo = () => {
  const {
    getCountNfts,
    selectNft,
    unselectNft,
    state: nftState,
    canBuyNft,
  } = useNft();
  const { state: authState } = useAuth();
  const { isConnected } = authState;
  const count = getCountNfts();
  const { data: balanceGLDT } = useLedgerUserBalance({ ledger: "GLDT" });

  const { isLoading, isSuccess, isError, error } = useGetAvailableGLDNFT();

  const handleOnChangeCount = (collectionIndex: number, type: string) => {
    if (type === "-") {
      unselectNft(collectionIndex);
    } else if (type === "+") {
      if (canBuyNft(collectionIndex, balanceGLDT?.number || 0))
        selectNft(collectionIndex);
    }
  };

  return (
    <div className={`border border-border rounded-xl md:p-6 p-4 opacity-100`}>
      <div className="text-gold font-semibold mb-2">To</div>
      {!isConnected && (
        <div className="flex justify-center items-center border border-border blur-[3px] bg-surface-2 py-3 px-4 rounded-xl mb-2 last:mb-0 sm:gap-12 gap-4">
          <div className="flex justify-between w-[260px]">
            <SelectNFTTitle collectionName="1g" />
            <SelectNFTCount
              collectionIndex={0}
              count={0}
              handleOnChangeCount={() => {}}
            />
          </div>
        </div>
      )}
      {isConnected && isLoading && <Loading />}
      {isConnected && nftState.isEmpty && <Empty />}
      {isConnected && isError && <Error error={error} />}
      {isConnected &&
        isSuccess &&
        !isLoading &&
        nftState.nfts.map((d, index) => {
          return (
            !d.isEmpty && (
              <div
                key={d.name}
                className="flex justify-center items-center border border-border bg-surface-2 py-3 px-1 sm:px-4 rounded-xl mb-2 last:mb-0 sm:gap-12 gap-4"
              >
                <div
                  className="flex justify-center items-center sm:gap-12 gap-4"
                  key={d.name}
                >
                  <SelectNFTTitle collectionName={d.name} />
                  <SelectNFTCount
                    collectionIndex={index}
                    count={count[index].selected}
                    handleOnChangeCount={handleOnChangeCount}
                  />
                  <div className="text-content/60 text-sm">
                    {count[index].selected} / {count[index].total}
                  </div>
                </div>
              </div>
            )
          );
        })}
    </div>
  );
};

export default ReverseSwapTo;
