const isReceiveAmountGreaterThanZero = (receive_amount: bigint) => {
  console.log("🔍 Validating receive amount:", receive_amount > 0n);
  return receive_amount > 0n;
};

export default isReceiveAmountGreaterThanZero;
