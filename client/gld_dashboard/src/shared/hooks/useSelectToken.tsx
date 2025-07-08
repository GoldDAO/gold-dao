import { useState, useCallback } from "react";
import { Token } from "@shared/utils/tokens";

type UseSelectTokenReturn = [Token, (token: Token) => void];

export const useSelectToken = (defaultValue: Token): UseSelectTokenReturn => {
  const [selectedToken, setSelectedToken] = useState<Token>(defaultValue);

  const onChange = useCallback((token: Token) => {
    setSelectedToken(token);
  }, []);

  return [selectedToken, onChange];
};
