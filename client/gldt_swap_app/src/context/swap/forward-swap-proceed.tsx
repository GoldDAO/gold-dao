import { createContext, useContext, ReactNode, useState, useMemo } from "react";
import { useForwardSwap } from "@hooks/gldt_swap";

const ForwardSwapProceedContext = createContext<ReturnType<
  typeof useForwardSwapProceedProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useForwardSwapProceed = () => {
  const context = useContext(ForwardSwapProceedContext);
  if (!context) {
    throw new Error(
      "useForwardSwapProceed must be used within a ForwardSwapProceedProvider"
    );
  }
  return context;
};

const useForwardSwapProceedProviderValue = () => {
  const [state, setState] = useState<{
    show: boolean;
    canCloseDialog: boolean;
  }>({
    show: false,
    canCloseDialog: true,
  });
  const forwardSwap = useForwardSwap();

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
      forwardSwap.reset();
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
      forwardSwap,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state, forwardSwap]
  );
  return value;
};

export const ForwardSwapProceedProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const contextValue = useForwardSwapProceedProviderValue();

  return (
    <ForwardSwapProceedContext.Provider value={contextValue}>
      {children}
    </ForwardSwapProceedContext.Provider>
  );
};
