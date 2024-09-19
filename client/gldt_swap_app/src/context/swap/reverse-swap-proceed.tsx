import { createContext, useContext, ReactNode, useState, useMemo } from "react";
import { useReverseSwap } from "@hooks/gldt_swap";

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
  }>({
    show: false,
    canCloseDialog: true,
  });
  const reverseSwap = useReverseSwap();

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
