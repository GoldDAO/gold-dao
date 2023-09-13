import { roundToNDigits } from './misc';

const url =
    'https://api2.yumi.io/gold/tradePrice?symbols=XAU&start_at=2023-08-01&end_at=2023-09-01';

const dayTimestamp = 24 * 3600 * 1000;
const thirtyDaysTimestamp = dayTimestamp * 30;

const gMultiplier = 28.345;

export const getGoldTimeserie = async () => {
    const start_date = formatDate(Date.now() - thirtyDaysTimestamp);
    const end_date = formatDate(Date.now() - dayTimestamp);
    const req = await fetch(url);
    const res = await req.json();
    return formatGoldTimeserie(res.rates);
};

const formatGoldTimeserie = (data) => {
    // const dates = Object.keys(data);
    // const formatedTimeserie = [];
    // dates.map((e) => {
    //     const ratio = data[e].XAU * gMultiplier;
    //     formatedTimeserie.push({
    //         date: e,
    //         XAU: 1,
    //         USD: roundToNDigits(data[e].USD / ratio, 2),
    //     });
    // });
    // return formatedTimeserie;
    return data;
};

export const formatDate = (timestamp) => {
    const date = new Date(timestamp);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
};
