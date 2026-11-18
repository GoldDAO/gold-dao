const E8sToLocaleString = ({
  value,
  tokenDecimals = 8,
  decimals = 2,
  locale = "en-US",
}: {
  value: bigint;
  tokenDecimals?: number;
  decimals?: number;
  locale?: string;
}) => {
  const num = Number(value) / 10 ** tokenDecimals;
  const result =
    value !== 0n
      ? num.toLocaleString(locale, {
          minimumFractionDigits: 0,
          // toLocaleString throws above its limit, and a ledger can report
          // any number of decimals.
          maximumFractionDigits: Math.min(decimals, 20),
        })
      : "0";
  return result;
};

export default E8sToLocaleString;
