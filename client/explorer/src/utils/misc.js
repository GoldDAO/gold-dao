export const roundToNDigits = (number, n) => {
    const digits = Math.pow(10, n);
    return Math.round(number * digits) / digits;
};

export const formatNumber = (number) => {
    const formattedNumber = number.replace(/\.?0+$/, '');
    return formattedNumber;
};
