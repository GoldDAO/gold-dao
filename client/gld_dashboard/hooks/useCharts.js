import { create } from 'zustand';
import { currentTimestamp, filterDates } from '../utils/functions';

export default create((set, get) => ({
  selectedDistance: 86400 * 31 * 6,
  gldGovSupply: { loading: true, data: [] },
  copyGldGovSupply: { loading: true, data: [] },
  gldGovTreasury: { loading: true, data: [] },
  copyGldGovTreasury: { loading: true, data: [] },
  stakersData: { loading: true, data: [] },
  copyStakersData: { loading: true, data: [] },
  setSelectedDistance: (selectedDistance) => {
    const distance = currentTimestamp() - selectedDistance;
    const filteredSupply = get().gldGovSupply.data.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );
    const copyGldGovSupply = filterDates(filteredSupply);

    const filteredTreasury = get().gldGovTreasury.data.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );
    const copyGldGovTreasury = filterDates(filteredTreasury);

    const filteredStakersData = get().stakersData.data.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );

    const copyStakersData = filterDates(filteredStakersData);

    return set({
      selectedDistance,
      copyGldGovSupply: { loading: false, data: copyGldGovSupply },
      copyGldGovTreasury: { loading: false, data: copyGldGovTreasury },
      copyStakersData: { loading: false, data: copyStakersData },
    });
  },
  setGldGovSupply: (data) => {
    const distance = currentTimestamp() - 86400 * 31 * 6; // 6 months in seconds;
    const filtered = data.filter(({ label }) => new Date(label) >= new Date(distance * 1000));
    const copyGldGovSupply = filterDates(filtered);

    return set({
      gldGovSupply: { loading: false, data },
      copyGldGovSupply: { loading: false, data: copyGldGovSupply },
    });
  },
  setGldGovTreasury: (data) => {
    const distance = currentTimestamp() - 86400 * 31 * 6; // 6 months in seconds;
    const filtered = data.filter(({ label }) => new Date(label) >= new Date(distance * 1000));
    const copyGldGovTreasury = filterDates(filtered);

    return set({
      gldGovTreasury: { loading: false, data },
      copyGldGovTreasury: { loading: false, data: copyGldGovTreasury },
    });
  },
  setStakersMetrics: (data) => {
    const distance = currentTimestamp() - 86400 * 31 * 6; // 6 months in seconds;
    const millisPerDay = 24 * 60 * 60 * 1000; // Number of milliseconds in a day
    const mappedData = data.map(([daysSinceEpoch, value]) => ({
      label: new Date(Number(daysSinceEpoch) * millisPerDay).toISOString().split('T')[0],
      value: Number(value.balance) / 1e8,
    }));

    console.log(mappedData);

    const filtered = mappedData.filter(({ label }) => new Date(label) >= new Date(distance * 1000));
    const copyStakersData = filterDates(filtered);
    console.log(copyStakersData);
    return set({
      stakersData: { loading: false, data: mappedData },
      copyStakersData: { loading: false, data: copyStakersData },
    });
  },
}));
