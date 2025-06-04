import { divideBy1e8 } from "@shared/utils/numbers";

const E8sToLocaleString = ({
  value,
  decimals = 3,
  locale = "en-US",
}: {
  value: bigint | (() => bigint);
  decimals?: number;
  locale?: string;
}) => {
  const result =
    value !== 0n
      ? divideBy1e8(value).toLocaleString(locale, {
          minimumFractionDigits: 0,
          maximumFractionDigits: decimals,
        })
      : "0";
  return result;
};

export default E8sToLocaleString;
