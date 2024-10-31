import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useMemo,
  useEffect,
} from "react";

import { useNft } from "@context/nft";

import { useReverseSwap } from "@hooks/gldt_swap";
import { useLedgerUserBalance } from "@hooks/ledger";

const ReverseSwapProceedContext = createContext<ReturnType<
  typeof useReverseSwapProceedProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useReverseSwapProceed = () => {
  const context = useContext(ReverseSwapProceedContext);
  if (!context) {
    throw new Error(
      "useReverseSwapProceed must be used within a ReverseSwapProceedProvider"
    );
  }
  return context;
};

const useReverseSwapProceedProviderValue = () => {
  const [state, setState] = useState<{
    show: boolean;
    canCloseDialog: boolean;
    isInsufficientGLDTFunds: boolean;
    canReverseSwap: boolean;
    totalSwapGLDT: string;
    balanceGLDT: string;
    countSelectedNfts: number;
  }>({
    show: false,
    canCloseDialog: true,
    isInsufficientGLDTFunds: false,
    canReverseSwap: false,
    totalSwapGLDT: "",
    balanceGLDT: "",
    countSelectedNfts: 0,
  });
  const reverseSwap = useReverseSwap();
  const { data: balanceGLDT } = useLedgerUserBalance({ ledger: "GLDT" });
  const { getCountSelectedNfts, getSelectedTotalGLDTWithFees } = useNft();

  const amountSelectedGLDTWithFees = getSelectedTotalGLDTWithFees();
  const countSelectedNfts = getCountSelectedNfts();

  const handleShow = (): void => {
    setState((prevState) => ({
      ...prevState,
      show: true,
    }));
  };

  const handleClose = (): void => {
    setState((prevState) => ({
      ...prevState,
      show: false,
    }));
    setTimeout(() => {
      reverseSwap.reset();
    }, 300);
  };

  const setCanCloseDialog = (canCloseDialog: boolean): void => {
    setState((prevState) => ({
      ...prevState,
      canCloseDialog,
    }));
  };

  useEffect(() => {
    if (balanceGLDT) {
      setState((prevState) => ({
        ...prevState,
        totalSwapGLDT: amountSelectedGLDTWithFees.string,
        balanceGLDT: balanceGLDT.string,
        countSelectedNfts,
      }));
      if (amountSelectedGLDTWithFees.number > balanceGLDT.number) {
        setState((prevState) => ({
          ...prevState,
          isInsufficientGLDTFunds: true,
        }));
      } else {
        setState((prevState) => ({
          ...prevState,
          isInsufficientGLDTFunds: false,
        }));
      }
    }
  }, [
    amountSelectedGLDTWithFees.number,
    amountSelectedGLDTWithFees.string,
    countSelectedNfts,
    balanceGLDT,
  ]);

  useEffect(() => {
    if (countSelectedNfts && !state.isInsufficientGLDTFunds) {
      setState((prevState) => ({
        ...prevState,
        canReverseSwap: true,
      }));
    } else {
      setState((prevState) => ({
        ...prevState,
        canReverseSwap: false,
      }));
    }
  }, [countSelectedNfts, state.isInsufficientGLDTFunds]);

  const value = useMemo(
    () => ({
      state,
      handleShow,
      handleClose,
      setCanCloseDialog,
      reverseSwap,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, reverseSwap]
  );
  return value;
};

export const ReverseSwapProceedProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const contextValue = useReverseSwapProceedProviderValue();

  return (
    <ReverseSwapProceedContext.Provider value={contextValue}>
      {children}
    </ReverseSwapProceedContext.Provider>
  );
};
