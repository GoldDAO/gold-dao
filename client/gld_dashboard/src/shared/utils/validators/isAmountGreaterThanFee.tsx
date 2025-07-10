const isAmountGreaterThanFee = (
  value: number,
  fee: bigint,
  decimals: number
) => {
  return BigInt(Math.round(value * 10 ** decimals)) >= fee;
};

export default isAmountGreaterThanFee;
