import { useAuth } from "@context/auth";

import { useNft } from "@context/nft";
import { useGetUserGLDNFT } from "@hooks/gld_nft";

import Card from "@components/shared/card/Base";
import {
  Count as SelectNFTCount,
  Title as SelectNFTTitle,
} from "@components/shared/select-nfts";

import Loading from "./Loading";
import Empty from "./Empty";
import Error from "./Error";

const ForwardSwapFrom = () => {
  const { getCountNfts, selectNft, unselectNft, state: nftState } = useNft();
  const { state: authState } = useAuth();
  const { isConnected } = authState;
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
      {isConnected && !isLoading && nftState.isEmpty && <Empty />}
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
    </>
  );
};

export default ForwardSwapFrom;
