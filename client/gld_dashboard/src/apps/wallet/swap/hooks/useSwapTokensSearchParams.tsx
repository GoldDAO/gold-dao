import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import {
  Token,
  TokenName,
  getTokenByDisplayName,
  TOKEN_SWAP_WHITELIST,
} from "@shared/utils/tokens";

type SwapModeResult =
  | {
      isSwap: false;
      tokenFrom: null;
      tokenTo: null;
      deleteSwapTokensSearchParams: () => void;
    }
  | {
      isSwap: true;
      tokenFrom: Token;
      tokenTo: Token;
      deleteSwapTokensSearchParams: () => void;
    };

const useSwapTokensSearchParams = (): SwapModeResult => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [isSwap, setIsSwap] = useState(false);
  const [tokenFrom, setTokenFrom] = useState<Token | null>(null);
  const [tokenTo, setTokenTo] = useState<Token | null>(null);

  const getParam = (key: string) => searchParams.get(key);

  const isValidSwapState = () => {
    const swap = getParam("swap");
    const token = getParam("token");
    const tokenSwapTo = getParam("receive_token");

    if (swap !== "true" || !token || !tokenSwapTo || token === tokenSwapTo) {
      return false;
    }

    if (
      !TOKEN_SWAP_WHITELIST.includes(token) ||
      !TOKEN_SWAP_WHITELIST.includes(tokenSwapTo)
    ) {
      return false;
    }

    const tokenFromObj = getTokenByDisplayName(token as TokenName);
    const tokenToObj = getTokenByDisplayName(tokenSwapTo as TokenName);
    return tokenFromObj !== null && tokenToObj !== null;
  };

  const clearSwapParams = () => {
    searchParams.delete("swap");
    searchParams.delete("receive_token");
    setSearchParams(searchParams);
  };

  const resetSwapState = () => {
    setIsSwap(false);
    setTokenFrom(null);
    setTokenTo(null);
  };

  const setValidSwapState = () => {
    setTokenFrom(
      getTokenByDisplayName(getParam("token") as TokenName) as Token
    );
    setTokenTo(
      getTokenByDisplayName(getParam("receive_token") as TokenName) as Token
    );
    setIsSwap(true);
  };

  const deleteSwapTokensSearchParams = () => {
    searchParams.delete("swap");
    searchParams.delete("receive_token");
    setSearchParams(searchParams);
  };

  useEffect(() => {
    if (isValidSwapState()) {
      setValidSwapState();
    } else {
      clearSwapParams();
      resetSwapState();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams]);

  if (isSwap && tokenFrom && tokenTo) {
    return {
      isSwap: true,
      tokenFrom,
      tokenTo,
      deleteSwapTokensSearchParams,
    };
  }

  return {
    isSwap: false,
    tokenFrom: null,
    tokenTo: null,
    deleteSwapTokensSearchParams,
  };
};

export default useSwapTokensSearchParams;
