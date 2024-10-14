import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useMemo,
  useEffect,
} from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";

import { useNft } from "@context/nft";
import { useLedgerUserBalance } from "@hooks/ledger";
import { useFetchTransferFeeNft } from "@hooks/gld_nft";
import { useTransferNFT } from "@hooks/gld_nft";
import { roundAndFormatLocale } from "@utils/numbers";

export interface TransferProceedNftState {
  to: string;
  balance: number | null;
  balanceE8s: number | null;
  balanceAfterTransfer: number;
  fee: number;
  totalTransferFee: { number: number; string: string };
  countSelectedNfts: number;
  canTransfer: boolean;
  isInsufficientOGYFunds: boolean;
}

const initialState: TransferProceedNftState = {
  to: "",
  balance: null,
  balanceE8s: null,
  balanceAfterTransfer: 0,
  fee: 0,
  totalTransferFee: { number: 0, string: "0" },
  countSelectedNfts: 0,
  canTransfer: false,
  isInsufficientOGYFunds: false,
};

const TransferProceedNftContext = createContext<ReturnType<
  typeof useTransferProceedNftProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useTransferProceedNft = () => {
  const context = useContext(TransferProceedNftContext);
  if (!context) {
    throw new Error(
      "useTransferProceedNft must be used within a TransferProceedNftProvider"
    );
  }
  return context;
};

const useTransferProceedNftProviderValue = () => {
  const queryClient = useQueryClient();
  const [state, setState] = useState<TransferProceedNftState>(initialState);
  const [show, setShow] = useState(false);
  const handleShowDialogConfirm = () => setShow(true);
  const handleCloseDialogConfirm = () => setShow(false);
  const mutation = useTransferNFT();
  const {
    getCollectionSelectedNFTs,
    getOneRandomNftId,
    getCountSelectedNfts,
    resetState: resetStateNft,
  } = useNft();
  const randomNftId = getOneRandomNftId(); // ? useful for the one call to transfer_fee
  const countSelectedNfts = getCountSelectedNfts();

  const balanceOGY = useLedgerUserBalance({ ledger: "OGY" });
  const fetchTransferFee = useFetchTransferFeeNft({
    nftId: randomNftId?.tokenId?.id_bigint,
    canister: randomNftId?.canister as string,
  });

  const form = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

  const handleSubmitForm = (data: { to: string }) => {
    setState((prevState) => ({
      ...prevState,
      to: data.to,
    }));
  };

  const handleTransfer = () => {
    const nfts = getCollectionSelectedNFTs();
    mutation.mutate(
      {
        to: state.to,
        nfts,
        fee: state.fee,
      }
      // {
      //   onSuccess: () => {
      //     resetStateNft();
      //     queryClient.invalidateQueries({
      //       queryKey: [`USER_FETCH_BALANCE_OGY`],
      //     });
      //     queryClient.invalidateQueries({
      //       queryKey: [`USER_GET_GLD_NFT_1G`],
      //     });
      //     queryClient.invalidateQueries({
      //       queryKey: [`USER_GET_GLD_NFT_10G`],
      //     });
      //     queryClient.invalidateQueries({
      //       queryKey: [`USER_GET_GLD_NFT_100G`],
      //     });
      //     queryClient.invalidateQueries({
      //       queryKey: [`USER_GET_GLD_NFT_1000G`],
      //     });
      //   },
      // }
    );
  };

  const handleReset = (): void => {
    resetStateNft();
    queryClient.invalidateQueries({
      queryKey: [`USER_FETCH_BALANCE_OGY`],
    });
    queryClient.invalidateQueries({
      queryKey: [`USER_GET_GLD_NFT_1G`],
    });
    queryClient.invalidateQueries({
      queryKey: [`USER_GET_GLD_NFT_10G`],
    });
    queryClient.invalidateQueries({
      queryKey: [`USER_GET_GLD_NFT_100G`],
    });
    queryClient.invalidateQueries({
      queryKey: [`USER_GET_GLD_NFT_1000G`],
    });
    setState(initialState);
    form.reset();
    mutation.reset();
  };

  useEffect(() => {
    setState((prevState) => ({
      ...prevState,
      countSelectedNfts,
    }));
  }, [countSelectedNfts]);

  useEffect(() => {
    if (
      fetchTransferFee.isSuccess &&
      fetchTransferFee.data &&
      randomNftId?.tokenId
    ) {
      setState((prevState) => ({
        ...prevState,
        fee: fetchTransferFee.data as number,
      }));
    }
  }, [fetchTransferFee.isSuccess, fetchTransferFee.data, randomNftId?.tokenId]);

  useEffect(() => {
    if (state.fee > 0 && countSelectedNfts) {
      const tFee = state.fee * countSelectedNfts;
      setState((prevState) => ({
        ...prevState,
        totalTransferFee: {
          number: tFee,
          string: roundAndFormatLocale({ number: tFee, decimals: 0 }),
        },
        balanceAfterTransfer: state.balance ? state.balance - tFee : 0,
      }));
    }
  }, [state.fee, countSelectedNfts, state.balance]);

  useEffect(() => {
    if (!countSelectedNfts || !randomNftId) {
      setState((prevState) => ({
        ...prevState,
        totalTransferFee: { number: 0, string: "0" },
      }));
    }
  }, [randomNftId, countSelectedNfts]);

  useEffect(() => {
    if (balanceOGY.isSuccess && balanceOGY.data) {
      setState((prevState) => ({
        ...prevState,
        balance: balanceOGY.data.number,
        balanceE8s: balanceOGY.data.e8s,
      }));
    }
  }, [balanceOGY.isSuccess, balanceOGY.data]);

  useEffect(() => {
    // ? testing + 29999.998
    if (state.balance && state.totalTransferFee.number > state.balance) {
      setState((prevState) => ({
        ...prevState,
        isInsufficientOGYFunds: true,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        isInsufficientOGYFunds: false,
      }));
    }
  }, [state.balance, state.totalTransferFee]);

  useEffect(() => {
    if (!state.isInsufficientOGYFunds && countSelectedNfts === 0) {
      setState((prevState) => ({
        ...prevState,
        canTransfer: false,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        canTransfer: true,
      }));
    }
  }, [state.isInsufficientOGYFunds, countSelectedNfts]);

  const value = useMemo(
    () => ({
      state,
      form,
      handleTransfer,
      handleReset,
      show,
      handleShowDialogConfirm,
      handleCloseDialogConfirm,
      mutation,
      handleSubmitForm,
      balanceOGY,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, form.formState, balanceOGY.data]
  );
  return value;
};

export const TransferProceedNftProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const contextValue = useTransferProceedNftProviderValue();

  return (
    <TransferProceedNftContext.Provider value={contextValue}>
      {children}
    </TransferProceedNftContext.Provider>
  );
};
