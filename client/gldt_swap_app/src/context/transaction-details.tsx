import { createContext, useContext, ReactNode, useMemo } from "react";
import { useParams, useSearchParams } from "react-router-dom";
import { useGetOneSwapById } from "@hooks/gldt_swap";

const TransactionDetailsContext = createContext<ReturnType<
  typeof useTransactionDetailsProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useTransactionDetails = () => {
  const context = useContext(TransactionDetailsContext);
  if (!context) {
    throw new Error(
      "useTransactionDetails must be used within a TransactionDetailsProvider"
    );
  }
  return context;
};

const useTransactionDetailsProviderValue = (nft_id: string, index: string) => {
  const { data, isSuccess, isLoading, isError } = useGetOneSwapById({
    nft_id,
    index,
  });

  const value = useMemo(
    () => ({
      data,
      isSuccess,
      isLoading,
      isError,
    }),
    [data, isSuccess, isLoading, isError]
  );

  return value;
};

export const TransactionDetailsProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const params = useParams();
  const [searchParams] = useSearchParams();
  const contextValue = useTransactionDetailsProviderValue(
    params.nft_id as string,
    searchParams.get("index") as string
  );

  return (
    <TransactionDetailsContext.Provider value={contextValue}>
      {children}
    </TransactionDetailsContext.Provider>
  );
};
