// import { useEffect, useRef, useState } from "react";
import { useWallet } from "@amerej/artemis-react";

import { useNft } from "@context/index";
import { useGetAvailableGLDNFT } from "@hooks/gld_nft";

import Card from "@components/shared/card/Base";
import {
  Count as SelectNFTCount,
  Title as SelectNFTTitle,
} from "@components/shared/select-nfts";

import Loading from "./Loading";
import Empty from "./Empty";
import Error from "./Error";
import { useUserBalanceGLDT } from "@hooks/gldt_ledger";

const ReverseSwapTo = () => {
  const { getCountNfts, selectNft, unselectNft, state: nftState, canBuyNft } = useNft();
  const { isConnected } = useWallet();
  const count = getCountNfts();
  const { data: balanceGLDT } = useUserBalanceGLDT();

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

      {!isConnected &&
        nftState.nfts.map((d, index) => {
          return (
            <Card key={d.name} className="mb-2 last:mb-0">
              <div
                className="flex justify-center items-center md:gap-12 gap-4"
                key={d.name}
              >
                <SelectNFTTitle collectionName={d.name} />
                <SelectNFTCount
                  collectionIndex={index}
                  count={0}
                  handleOnChangeCount={() => {}}
                />
              </div>
            </Card>
          );
        })}
      {isConnected && isLoading && <Loading />}
      {isConnected && nftState.isEmpty && <Empty />}
      {isConnected && isError && <Error error={error} />}
      {isConnected &&
        isSuccess &&
        !isLoading &&
        nftState.nfts.map((d, index) => {
          return (
            !d.isEmpty && (
              <Card key={d.name} className="mb-2 last:mb-0">
                <div
                  className="flex justify-center items-center md:gap-12 gap-4"
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
              </Card>
            )
          );
        })}
    </div>
  );
};

export default ReverseSwapTo;
