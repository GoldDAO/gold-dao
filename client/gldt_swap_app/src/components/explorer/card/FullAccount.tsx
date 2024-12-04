import CopyToClipboard from "@components/shared/button/CopyToClipboard";
import { useGetFullAccount } from "@hooks/useGetFullAccount";
import { LoaderSpin } from "@components/ui";

export const FullAccount = ({
  owner,
  subaccount,
  className,
}: {
  owner: string;
  subaccount?: string | undefined;
  className?: string;
}) => {
  const { data, isSuccess, isLoading, isError } = useGetFullAccount({
    owner,
    subaccount,
  });
  return (
    <div
      className={`border border-border rounded-xl bg-surface p-6 ${className}`}
    >
      <div className="text-center lg:text-left mb-4">Full account</div>

      {(isLoading || isError) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}

      {isSuccess && (
        <div className="w-full flex items-center justify-center lg:justify-start">
          <button
            data-tooltip-id="tooltip"
            data-tooltip-content={data}
            className="mr-2 truncate text-2xl font-semibold"
          >
            {data}
          </button>
          <CopyToClipboard value={data} />
        </div>
      )}
    </div>
  );
};
