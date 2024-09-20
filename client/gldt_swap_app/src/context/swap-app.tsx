import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
} from "react";
import { useSearchParams } from "react-router-dom";

// eslint-disable-next-line react-refresh/only-export-components
export enum View {
  SWAP,
  TRANSFER,
}

export interface SwapAppState {
  view: View;
}

const initialState: SwapAppState = {
  view: View.SWAP,
};

const SwapAppContext = createContext<{
  state: SwapAppState;
  setView: (view: number) => void;
}>({
  state: initialState,
  setView: () => {},
});

// eslint-disable-next-line react-refresh/only-export-components
export const useSwapApp = () => {
  const context = useContext(SwapAppContext);
  if (!context) {
    throw new Error("useSwapApp must be used within a SwapAppProvider");
  }
  return context;
};

export const SwapAppProvider = ({ children }: { children: ReactNode }) => {
  const [state, setState] = useState<SwapAppState>(initialState);
  const [searchParams, setSearchParams] = useSearchParams();
  const viewSearchParams = Number(searchParams.get("view"));

  const setView = (view: View): void => {
    const _view = view === View.TRANSFER ? View.TRANSFER : View.SWAP;
    if (view === View.TRANSFER) {
      searchParams.delete("mode");
    } else {
      searchParams.delete("token");
    }
    setState((prevState) => ({
      ...prevState,
      view: _view,
    }));
    searchParams.set("view", _view.toString());
    setSearchParams(searchParams);
  };

  useEffect(() => {
    setView(viewSearchParams);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [viewSearchParams]);

  return (
    <SwapAppContext.Provider
      value={{
        state,
        setView,
      }}
    >
      {children}
    </SwapAppContext.Provider>
  );
};
