import millifyPackage from "millify";
export const locale = "en-US";

export const divideBy1e8 = (number: number | bigint | string | (() => bigint)) =>
  Number(number) / 1e8;

interface roundAndFormatLocaleParams {
  number: number;
  locale?: string;
  decimals?: number;
}

export const roundAndFormatLocale = ({
  number,
  locale = "en-US",
  decimals = 3,
}: roundAndFormatLocaleParams) => {
  return Number(number.toFixed(decimals)).toLocaleString(locale);
};

export const numberToE8s = (value: string) => {
  return BigInt(Math.round(parseFloat(value) * 1e8));
};

export const millify = (value: number, precision?: number) =>
  millifyPackage(value, { precision: precision ?? 3, locales: locale });

export const numberToLocaleString = ({
  value,
  decimals = 3,
  locale = "en-US",
}: {
  value: number;
  decimals?: number;
  locale?: string;
}) => {
  if (value === 0) return "0";
  return value.toLocaleString(locale, {
    minimumFractionDigits: 0,
    maximumFractionDigits: decimals,
  });
};
