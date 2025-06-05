import useUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";

const TotalCountUserNFTs = () => {
  const { data: nfts, isSuccess: isSuccessFetchUserNFTs } = useUserNFTMetrics();

  return (
    <div className="flex flex-col items-center gap-2">
      <div className="text-2xl xl:text-4xl font-semibold">
        {isSuccessFetchUserNFTs ? (
          <div className="flex items-center gap-2">
            {nfts.totalCount}
            <div className="text-content/60 font-normal">NFTs</div>
          </div>
        ) : (
          <div>Loading...</div>
        )}
      </div>
      {isSuccessFetchUserNFTs ? (
        <div>{nfts.totalGrams} grams of Gold ($todo)</div>
      ) : (
        <div>Loading...</div>
      )}
    </div>
  );
};

export default TotalCountUserNFTs;
