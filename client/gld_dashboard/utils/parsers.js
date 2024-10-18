import { Principal } from '@dfinity/principal';

export const shortPrincipal = (
  p = '3px7p-abe4z-r4yl3-gek3i-hso7z-hj27l-vmjho-ytvbj-kyaze-u4btl-fqe',
) => `${p.substring(0, 5)}...${p.substring(p.length - 3, p.length)}`;

export const currentPage = (path) => (path === '/rewards' ? 'My Rewards' : 'Dashboard');

export const parseNumbers = (n = 0, toFixed = false) => {
  let number = n;
  if (typeof n === 'string') number = parseFloat(n);

  return typeof toFixed === 'number'
    ? parseFloat(number.toFixed(toFixed))
      .toLocaleString('en')
      .replace(/,/g, '’')
      // .replace('.', ',')
    : number.toLocaleString('en').replace(/,/g, '’');
};

export const p = (principal) => Principal.fromText(principal);

export const convertDate = (date) => {
  const milisecondsDate = date / 1000000;

  const newDate = new Date(milisecondsDate);

  const year = newDate.getUTCFullYear();
  const month = `0${newDate.getUTCMonth() + 1}`.slice(-2);
  const day = `0${newDate.getUTCDate()}`.slice(-2);
  const hours = `0${newDate.getUTCHours()}`.slice(-2);
  const minutes = `0${newDate.getUTCMinutes()}`.slice(-2);
  const seconds = `0${newDate.getUTCSeconds()}`.slice(-2);

  const parsedDate = `${year}-${month}-${day}, ${hours}:${minutes}:${seconds} UTC`;

  return parsedDate;
};

export const parsedNeuronId = (arrId = []) => {
  const hex = arrId
    .map((number) => (number / 16).toString(16).padStart(4, '0'))
    .join('');

  console.log(hex); // "0001f199272d5d606362e6ecd855f079efc90f4779d3ae10d943f74e42ff99e0"
  // 010992700606362000550790004779001004300420990

  return hex;
};

export const parseCycles = (cycles) => cycles / 10 ** 12;

export const parseTimestamp = (timestamp) => {
  const date = new Date(timestamp * 1000);

  const year = date.getFullYear();
  const month = `0${date.getMonth() + 1}`.slice(-2);
  const day = `0${date.getDate()}`.slice(-2);

  const formatedDate = `${year}-${month}-${day}`;

  return formatedDate;
};
