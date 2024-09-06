import { LoaderSpin } from "@components/ui";
import { useTransactionDetails } from "@context/index";

const Details = () => {
  const { isSuccess, data, isLoading, isError } = useTransactionDetails();

  return (
    <div className="border border-border rounded-xl bg-surface p-4 sm:p-6">
      {isSuccess && data && (
        <>
          <div className="mb-4 font-semibold">Details</div>
          <div className="">
            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60">Type</div>
              <div className="text-content/60">{data.label}</div>
            </div>

            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60">Date/Hour</div>
              <div className="text-content/60">{data.created_at}</div>
            </div>

            <div className="border-b border-border py-4">
              <div className="flex justify-between items-center mb-2">
                <div className="font-semibold text-content/60">
                  Total grams of gold
                </div>
                <div className="font-semibold text-content/60">
                  {data.nft_value}g
                </div>
              </div>
              <div className="flex justify-between items-center">
                <div className="text-sm text-content/60">Serial number</div>
                <div className="text-sm text-content/60">
                  {data.nft_id_string}
                </div>
              </div>
            </div>

            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60">
                Swapped amount
              </div>
              <div className="font-semibold text-content/60">
                {data.gldt_value} GLDT
              </div>
            </div>

            <div className="flex justify-between items-center py-4">
              <div className="font-semibold text-content/60">
                Conversion fee
              </div>
              <div className="text-content/60">
                {data.gldt_value / data.gldt_value} GLDT
              </div>
            </div>
          </div>
        </>
      )}
      {(isLoading || isError) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};

export default Details;
