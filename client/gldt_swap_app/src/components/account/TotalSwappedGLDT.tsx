import { useGetUserNftsMetrics } from "@hooks/gld_nft";
import { LoaderSpin } from "@components/ui";

const TotalSwappedGLDT = ({ className }: { className?: string }) => {
  const { data, isSuccess, isLoading, isError } = useGetUserNftsMetrics();

  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-6">
        <div className="mb-2 font-light text-content/60 text-center sm:text-left">
          Total of GLDT swapped
        </div>
        {isSuccess && (
          <div className="flex items-center justify-center sm:justify-start gap-2">
            <img className="flex-none h-8" src={`/gldt_logo.svg`} />
            <div className="font-semibold text-4xl">
              {data?.totalCountGLDT ?? 0}
            </div>
            <div className="font-semibold text-xl">GLDT</div>
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

export default TotalSwappedGLDT;
