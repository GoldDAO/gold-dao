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
export enum Token {
  GLDT,
  GLD_NFT,
  OGY,
}

export interface TransferState {
  token: Token;
}

const initialState: TransferState = {
  token: Token.GLDT,
};

const TransferContext = createContext<ReturnType<
  typeof useTransferProviderValue
> | null>(null);

// eslint-disable-next-line react-refresh/only-export-components
export const useTransfer = () => {
  const context = useContext(TransferContext);
  if (!context) {
    throw new Error("useTransfer must be used within a TransferProvider");
  }
  return context;
};

const useTransferProviderValue = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const tokenSearchParams = Number(searchParams.get("token"));
  const [state, setState] = useState<TransferState>(initialState);

  const setToken = (token: Token): void => {
    let _token = Token.OGY;
    if (token === Token.GLDT) {
      _token = Token.GLDT;
    } else if (token === Token.GLD_NFT) {
      _token = Token.GLD_NFT;
    }

    setState((prevState) => ({
      ...prevState,
      token: _token,
    }));
    searchParams.set("token", token.toString());
    setSearchParams(searchParams);
  };

  const resetTransfer = (): void => {
    setState(initialState);
  };
  useEffect(() => {
    setToken(tokenSearchParams);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [tokenSearchParams]);

  const value = useMemo(
    () => ({
      state,
      setToken,
      resetTransfer,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state]
  );
  return value;
};

export const TransferProvider = ({ children }: { children: ReactNode }) => {
  const contextValue = useTransferProviderValue();

  return (
    <TransferContext.Provider value={contextValue}>
      {children}
    </TransferContext.Provider>
  );
};
