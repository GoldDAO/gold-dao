/* eslint-disable no-mixed-operators */
/* eslint-disable no-shadow */
import moment from 'moment';
import { parseNumbers } from './parsers';

export const currentTimestamp = () => Math.round(new Date().getTime() / 1000);

export const calculateTimestamp = (timestamp) => {
  let distanceTimestamp;

  if (!timestamp) {
    const now = new Date();
    const firstDay = new Date(now.getFullYear(), 0, 1);
    const milisecondsDistance = (now - firstDay) / 1000;
    distanceTimestamp = Math.floor(milisecondsDistance);
  } else if (timestamp === 'ALL') {
    distanceTimestamp = new Date(2000, 0, 1) / 1000;
  } else {
    distanceTimestamp = timestamp;
  }

  return distanceTimestamp;
};

export const verifyTimestamp = (timestamp) => {
  const rounded = [86400, 604800, 2678400, 8035200, 16070400, 32140600];

  if (rounded.includes(timestamp)) return timestamp;
  // FIXME con esto falla el 6M. ver de aaj modificarlo

  const distances = rounded.map((valor) => Math.abs(timestamp / 4 - valor));

  const indexMin = distances.indexOf(Math.min(...distances));
  return rounded[indexMin];
};

export const filterDates = (arr = []) => {
  let length = 10;
  if (arr.length < 12) length = arr.length - 2;

  const selected = [arr[0]];

  const distance = Math.floor((arr.length - 2) / length);

  for (let i = 1; i <= length; i += 1) {
    selected.push(arr[i * distance]);
  }

  selected.push(arr[arr.length - 1]);

  return selected;
};

export const uint8ArrayToHexString = (uint8Array) => Array.prototype.map
// eslint-disable-next-line no-bitwise
  .call(uint8Array, (byte) => `0${(byte & 0xff).toString(16)}`.slice(-2))
  .join('');

export const truncateNeuronId = (string, start = 7, end = 7) => {
  const hexString = uint8ArrayToHexString(string);
  if (!hexString) return null;
  if (hexString.length <= 25) return hexString;
  const modifiedString = `${hexString.substring(0, start)}...${hexString.slice(-end)}`;
  return modifiedString;
};

export const truncatePrincipal = (string) => {
  if (!string) return null;
  if (string.length <= 25) return string;
  const modifiedString = `${string.substring(0, 5)}...${string.slice(-3)}`;
  return modifiedString;
};

export const copyContent = async (contentToCopy, setCopyState) => {
  try {
    await navigator.clipboard.writeText(contentToCopy);
    setCopyState(true);
  } catch (error) {
    console.error('Error copying content: ', error);
  }
};

export function elapsedTime(seconds) {
  if (seconds <= 0 || !seconds) {
    return '-';
  }
  return moment.duration(seconds, 'seconds').humanize();
}

export function formatDateFromSeconds(seconds) {
  const milliseconds = seconds * 1000;
  const date = moment(milliseconds);

  const formattedDate = date.format('YYYY-MM-DD, HH:mm:ss [UTC]');

  return formattedDate;
}

export const calculateVotingPower = (values) => {
  try {
    const {
      cachedNeuronStakeE8s,
      stakedMaturiryE8sEquivalent,
      age,
      maxNeuronAgeForAgeBonus,
      maxAgeBonusPercentage,
      maxDissolveDelaySeconds,
      dissolveState,
      maxDissolveDelayBonusPercentage,
    } = values;
    const stakedAmount = Number(cachedNeuronStakeE8s || 0) / 1e8;
    const stakedMaturity = Number(stakedMaturiryE8sEquivalent || 0) / 1e8;

    let ageBonus = 0;
    let dissolveDelayBonus = 0;
    let dissolveDelay = 0;
    const currentTimestamp = Math.round(new Date().getTime() / 1000);
    if (dissolveState.DissolveDelaySeconds) {
      dissolveDelay = Number(dissolveState.DissolveDelaySeconds) || 0;
      ageBonus = ((age / Number(maxNeuronAgeForAgeBonus)) * Number(maxAgeBonusPercentage)) / 100;
      dissolveDelayBonus = (Number(dissolveState.DissolveDelaySeconds)
          / (maxDissolveDelaySeconds * maxDissolveDelayBonusPercentage))
        * 100;
    } else {
      dissolveDelay = (Number(dissolveState.WhenDissolvedTimestampSeconds) - currentTimestamp) || 0;
      dissolveDelayBonus = ((Number(dissolveState.WhenDissolvedTimestampSeconds) - currentTimestamp)
          / (maxDissolveDelaySeconds * maxDissolveDelayBonusPercentage))
        * 100;
    }

    const votingPower = (stakedAmount + stakedMaturity) * (1 + ageBonus) * (1 + dissolveDelayBonus);

    return {
      votingPower: parseNumbers(votingPower.toFixed(2)),
      ageBonus,
      dissolveDelayBonus,
      dissolveDelay,
    };
  } catch (error) {
    console.log('error voting power', error);
    return 0;
  }
};

export const hexStringToUint8Array = (hexString) => {
  const bytes = [];
  for (let i = 0; i < hexString.length; i += 2) {
    bytes.push(parseInt(hexString.substr(i, 2), 16));
  }
  return new Uint8Array(bytes);
};

export const arrayToHex = (array) => Buffer.from(array).toString('hex');

export const neuronState = (dissolveState) => {
  let state;
  const currentTimestampSeconds = new Date().getTime() / 1000;
  if (dissolveState.DissolveDelaySeconds) {
    state = 'Not dissolving';
  } else if (dissolveState.WhenDissolvedTimestampSeconds > currentTimestampSeconds) {
    state = 'Dissolving';
  } else {
    state = 'Dissolved';
  }
  return state;
};
