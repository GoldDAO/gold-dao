const TokenValueToLocaleString = ({
  value,
  tokenDecimals = 8,
  decimals = 3,
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
          maximumFractionDigits: decimals,
        })
      : "0";
  return result;
};

export default TokenValueToLocaleString;
