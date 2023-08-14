import { roundToNDigits } from './misc';

const YumiMetalApiKey = 'pj9uxpzo9szpk9weqz3n5ny3gg0za54685al60kwsn129lz52m23nmghu19g';
const baseApiUrl = 'https://metals-api.com/api';

const dayTimestamp = 24 * 3600 * 1000;
const thirtyDaysTimestamp = dayTimestamp * 30;

const gMultiplier = 28.345;

export const getGoldTimeserie = async () => {
  const start_date = formatDate(Date.now() - thirtyDaysTimestamp);
  const end_date = formatDate(Date.now() - dayTimestamp);
  const params = `/timeseries?access_key=${YumiMetalApiKey}&start_date=${start_date}&end_date=${end_date}&symbols=XAU`;
  const req = await fetch(baseApiUrl + params);
  const res = await req.json();
  return formatGoldTimeserie(res.rates);
};

const formatGoldTimeserie = (data) => {
  const dates = Object.keys(data);
  const formatedTimeserie = [];
  dates.map((e) => {
    const ratio = data[e].XAU * gMultiplier;
    formatedTimeserie.push({
      date: e,
      XAU: 1,
      USD: roundToNDigits(data[e].USD / ratio, 2),
    });
  });
  return formatedTimeserie;
};

export const formatDate = (timestamp) => {
  const date = new Date(timestamp);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
};
