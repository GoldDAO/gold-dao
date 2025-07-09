const useSendAmountInputValidation = (
  balance: bigint,
  fee: bigint,
  decimals: number
) => {
  const getE8sValue = (value: number) => {
    return BigInt(Math.round(value * 10 ** decimals));
  };

  const isInsufficientFunds = (value: number) => {
    if (value === 0) return true;
    return getE8sValue(value) + fee <= balance;
  };

  const isAmountGreaterThanFee = (value: number) => {
    if (value === 0) return true;
    return getE8sValue(value) >= fee;
  };

  const isAmountGreaterThanZero = (value: number) => value > 0;

  return {
    isInsufficientFunds,
    isAmountGreaterThanFee,
    isAmountGreaterThanZero,
  };
};

export default useSendAmountInputValidation;
