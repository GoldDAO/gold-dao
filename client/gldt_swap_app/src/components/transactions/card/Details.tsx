import { LoaderSpin } from "@components/ui";
import { useTransactionDetails } from "@context/index";

import CopyToClipboard from "@components/shared/button/CopyToClipboard";

const Details = () => {
  const { isSuccess, data, isLoading, isError } = useTransactionDetails();

  return (
    <div className="border border-border rounded-xl bg-surface p-4 md:p-6">
      {isSuccess && data && (
        <>
          <div className="mb-4 font-semibold">Details</div>
          <div className="">
            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60">Swap Index</div>
              <div className="text-content/60">{data.index}</div>
            </div>

            <div className="flex justify-between items-center border-b border-border py-4 break-all">
              <div className="font-semibold text-content/60">NFT NAT ID</div>
              <div className="max-w-48">
                <div className="flex items-center truncate">
                  <div className="flex ml-8 items-center truncate">
                    <div
                      className="truncate text-content/60"
                      data-tooltip-id="tooltip"
                      data-tooltip-content={data.nft_id}
                    >
                      {data.nft_id}
                    </div>
                    <CopyToClipboard value={data.nft_id} />
                  </div>
                </div>
              </div>
            </div>
            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60">Type</div>
              <div className="text-content/60 text-left">{data.label}</div>
            </div>

            <div className="flex justify-between items-center border-b border-border py-4">
              <div className="font-semibold text-content/60 min-w-32">
                Date/Hour
              </div>
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

            <div className="flex justify-between items-center py-4">
              <div className="font-semibold text-content/60">
                Swapped amount
              </div>
              <div className="font-semibold text-content/60">
                {data.gldt_value} GLDT
              </div>
            </div>

            {data.type !== "forward" && (
              <div className="flex justify-between items-center border-t border-border py-4">
                <div className="font-semibold text-content/60">
                  Conversion fee
                </div>
                <div className="text-content/60">
                  {data.gldt_value / data.gldt_value} GLDT
                </div>
              </div>
            )}
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
