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
  holdersData: { loading: true, data: [] },
  copyHoldersData: { loading: true, data: [] },
  burnData: { loading: false, data: [] },
  copBurnData: { loading: false, data: [] },
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

    const filteredHoldersData = get().holdersData.data.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );
    const copyHoldersData = filterDates(filteredHoldersData);

    const filteredBurnData = get().burnData.data.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );
    const copyBurnData = filterDates(filteredBurnData);

    return set({
      selectedDistance,
      copyGldGovSupply: { loading: false, data: copyGldGovSupply },
      copyGldGovTreasury: { loading: false, data: copyGldGovTreasury },
      copyStakersData: { loading: false, data: copyStakersData },
      copyHoldersData: { loading: false, data: copyHoldersData },
      copyBurnData: { loading: false, data: copyBurnData },
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
    const filtered = mappedData.filter(({ label }) => new Date(label) >= new Date(distance * 1000));
    const copyStakersData = filterDates(filtered);
    return set({
      stakersData: { loading: false, data: mappedData },
      copyStakersData: { loading: false, data: copyStakersData },
    });
  },
  setHoldersData: (data) => {
    const distance = currentTimestamp() - 86400 * 31 * 6; // 6 months in seconds;
    const mappedData = data.map(
      ({
        end_time: endTime,
        total_unique_accounts: totalAccs,
        total_unique_principals: totalPrins,
      }) => ({
        label: new Date(Number(endTime) / 1000000).toISOString().split('T')[0],
        value: Number(totalAccs) + Number(totalPrins),
      }),
    );
    const filtered = mappedData.filter(({ label }) => new Date(label) >= new Date(distance * 1000));
    const copyHoldersData = filterDates(filtered);

    return set({
      holdersData: { loading: false, data: mappedData },
      copyHoldersData: { loading: false, data: copyHoldersData },
    });
  },
  setBurnData: (data) => {
    const distance = currentTimestamp() - 86400 * 31 * 6; // 6 months in seconds;
    const transformedData = data.map((subarr) => ({
      value: (Number(subarr[1])) / 1e8,
      label: new Date(Number(subarr[0]) * 1000).toISOString().split('T')[0],
    }));

    const cumulativeData = transformedData.reduce(
      (
        accumulator,
        current,
        index,
      ) => {
        const previousValue = index === 0 ? 0 : accumulator[index - 1].value;
        const cumulativeValue = previousValue + current.value;
        accumulator.push({ label: current.label, value: cumulativeValue });
        return accumulator;
      },
      [],
    );
    const filtered = cumulativeData.filter(
      ({ label }) => new Date(label) >= new Date(distance * 1000),
    );
    const copyBurnData = filterDates(filtered);

    return set({
      burnData: { loading: false, data: cumulativeData },
      copyBurnData: { loading: false, data: copyBurnData },
    });
  },
}));
