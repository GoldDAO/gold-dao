import { useAuth } from "@auth/index";
import { useNft } from "@context/nft";
import { useGetUserGLDNFT } from "@hooks/gld_nft";

import {
  Count as SelectNFTCount,
  Title as SelectNFTTitle,
} from "@components/shared/nft-select";

import Loading from "@components/shared/user-nft/Loading";
import Empty from "@components/shared/user-nft/Empty";
import Error from "@components/shared/user-nft/Error";

const ForwardSwapFrom = () => {
  const { getCountNfts, selectNft, unselectNft, state: nftState } = useNft();
  const { isConnected } = useAuth();
  const count = getCountNfts();

  const { isLoading, isSuccess, isError, error } = useGetUserGLDNFT();

  const handleOnChangeCount = (collectionIndex: number, type: string) => {
    if (type === "-") {
      unselectNft(collectionIndex);
    } else if (type === "+") {
      selectNft(collectionIndex);
    }
  };

  return (
    <>
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
      {isConnected && !isLoading && nftState.isEmpty && <Empty />}
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
                <div className="flex items-center justify-between w-[260px]">
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
    </>
  );
};

export default ForwardSwapFrom;
