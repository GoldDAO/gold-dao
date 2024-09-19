import { createContext, useContext, ReactNode, useMemo } from "react";
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
  nft_id,
  index,
}: {
  children: ReactNode;
  nft_id: string;
  index: string;
}) => {
  const contextValue = useTransactionDetailsProviderValue(nft_id, index);

  return (
    <TransactionDetailsContext.Provider value={contextValue}>
      {children}
    </TransactionDetailsContext.Provider>
  );
};
