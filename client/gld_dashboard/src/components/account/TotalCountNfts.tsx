import { useGetUserNftsMetrics } from "@hooks/gld_nft";
import { LoaderSpin } from "@components/index";

const TotalCountNfts = ({ className }: { className?: string }) => {
  const { data, isSuccess, isLoading, isError } = useGetUserNftsMetrics();

  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface-primary p-6">
        <div className="mb-2 font-light text-content/60 text-center sm:text-left">
          Total count of NFTs owned
        </div>
        {isSuccess && (
          <div className="flex items-center justify-center sm:justify-start gap-4">
            <img className="flex-none h-8" src={`/nft_logo.svg`} />
            <div className="font-semibold text-4xl">
              {data?.totalCountNFT ?? 0}
            </div>
          </div>
        )}

        {(isLoading || isError) && (
          <div className="flex justify-center">
            <LoaderSpin />
          </div>
        )}
      </div>
    </div>
  );
};

export default TotalCountNfts;
