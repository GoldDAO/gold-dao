import GoBack from "@components/shared/button/GoBack";
import DetailsHeader from "@components/transactions/header/Details";
import SwapCard from "@components/transactions/card/swap/Swap";
import DetailsCard from "@components/transactions/card/Details";

import { useTransactionDetails } from "@context/index";

export const TransactionDetails = () => {
  const { isSuccess, data, isLoading, isError } = useTransactionDetails();

  return (
    <div className="container mx-auto max-w-3xl mt-4 sm:mt-8 flex flex-col gap-4">
      <GoBack className="mb-4" />
      {(isLoading || isError) && (
        <>
          <div className="mb-4">
            <div className="text-4xl font-semibold text-gold">Transaction</div>
            <div className="text-4xl">Details</div>
          </div>
          <div className="border border-border rounded-xl bg-surface p-4 xl:p-16">
            <div className="flex justify-center mb-8">
              <img
                className="animate-beat h-16 w-16"
                src="/gldt_logo.svg"
                alt="Beat animation GLDT logo."
              />
            </div>

            <div className="text-center font-semibold">
              Loading transaction details...
            </div>
          </div>
        </>
      )}
      {isSuccess && data && (
        <>
          <DetailsHeader data={data} className="mb-4" />
          <SwapCard data={data} />
          <DetailsCard data={data} />
        </>
      )}
    </div>
  );
};
