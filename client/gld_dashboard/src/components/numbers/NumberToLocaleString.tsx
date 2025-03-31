const NumberToLocaleString = ({
  value,
  decimals = 3,
  locale = "en-US",
}: {
  value: number | (() => number);
  decimals?: number;
  locale?: string;
}) => {
  const result =
    value !== 0
      ? value.toLocaleString(locale, {
          minimumFractionDigits: 0,
          maximumFractionDigits: decimals,
        })
      : "0";
  return result;
};

export default NumberToLocaleString;
