import { useGetUserNftsMetrics } from "@hooks/gld_nft";
import { LoaderSpin } from "@components/ui";

const TotalWeightNfts = ({ className }: { className?: string }) => {
  const { data, isSuccess, isLoading, isError } = useGetUserNftsMetrics();

  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-6">
        <div className="mb-2 font-light text-content/60 text-center sm:text-left">
          Total of NFTs weight
        </div>
        {isSuccess && (
          <div className="flex items-center justify-center sm:justify-start gap-4">
            <img className="flex-none h-8" src={`/gold-bars/${1}g.svg`} />
            <div className="font-semibold text-4xl">
              {data?.totalCountWeight ?? 0}g
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

export default TotalWeightNfts;
