import { useParams, useSearchParams } from "react-router-dom";

import { TransactionDetailsProvider } from "@context/index";

import TransactionDetails from "@components/transactions/navigation/TransactionDetails";
import DetailsHeader from "@components/transactions/header/Details";
import SwapCard from "@components/transactions/card/swap/Swap";
import DetailsCard from "@components/transactions/card/Details";

const Details = () => {
  const params = useParams();
  const [searchParams] = useSearchParams();

  return (
    <TransactionDetailsProvider
      nft_id={params.nft_id as string}
      index={searchParams.get("index") as string}
    >
      <div className="container mx-auto max-w-4xl mt-4 sm:mt-8 flex flex-col gap-4">
        <TransactionDetails nft_id={params.nft_id as string} />
        <DetailsHeader className="my-4" />
        <SwapCard />
        <DetailsCard />
      </div>
    </TransactionDetailsProvider>
  );
};

export default Details;
