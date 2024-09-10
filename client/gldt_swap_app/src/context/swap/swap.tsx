import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
  useMemo,
} from "react";
import { useSearchParams } from "react-router-dom";

// eslint-disable-next-line react-refresh/only-export-components
export enum Mode {
  FORWARD,
  REVERSE,
}

export interface SwapState {
  mode: Mode;
}

const initialState: SwapState = {
  mode: Mode.FORWARD,
};

const SwapContext = createContext<ReturnType<
  typeof useSwapProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useSwap = () => {
  const context = useContext(SwapContext);
  if (!context) {
    throw new Error("useSwap must be used within a SwapProvider");
  }
  return context;
};

const useSwapProviderValue = () => {
  const [state, setState] = useState<SwapState>(initialState);
  const [searchParams, setSearchParams] = useSearchParams();
  const modeSearchParams = Number(searchParams.get("mode"));

  const setMode = (mode: Mode): void => {
    const _mode = mode === Mode.FORWARD ? Mode.FORWARD : Mode.REVERSE;
    setState((prevState) => ({
      ...prevState,
      mode: _mode,
    }));
    searchParams.set("mode", _mode.toString());
    setSearchParams(searchParams);
  };

  const resetState = (): void => {
    setState(initialState);
  };

  useEffect(() => {
    setMode(modeSearchParams);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [modeSearchParams]);

  const value = useMemo(
    () => ({
      state,
      setMode,
      resetState,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state]
  );
  return value;
};

export const SwapProvider = ({ children }: { children: ReactNode }) => {
  const contextValue = useSwapProviderValue();

  return (
    <SwapContext.Provider value={contextValue}>{children}</SwapContext.Provider>
  );
};
