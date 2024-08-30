export const locale = 'en-US';

export const divideBy1e8 = (number: number | bigint | string) => Number(number) / 1e8;

interface roundAndFormatLocaleParams {
    number : number;
    locale?: string;
    decimals?: number;
}

export const roundAndFormatLocale = ({number, locale="en-US", decimals=2}: roundAndFormatLocaleParams) => {
    return Number(number.toFixed(decimals)).toLocaleString(locale);
};

export const numberToE8s = (value:string) => {
    return BigInt(Math.round(parseFloat(value) * 1e8))
}
