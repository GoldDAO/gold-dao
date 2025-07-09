const isInsufficientFunds = (
  value: number,
  balance: bigint,
  fee: bigint,
  decimals: number
) => {
  return BigInt(Math.round(value * 10 ** decimals)) + fee <= balance;
};

export default isInsufficientFunds;
