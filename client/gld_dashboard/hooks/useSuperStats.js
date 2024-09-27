import useActor from './useActor';
import useCharts from './useCharts';

// Define the specific date
const startDate = new Date('2023-11-13'); // minus 1 month
const today = new Date();
const timeDifference = today - startDate;
const projectStartDateDiffDays = Math.floor(timeDifference / (1000 * 60 * 60 * 24));

console.log(`${projectStartDateDiffDays} days since ${startDate.toDateString()}`);

const useSuperStats = () => {
  const [superStats] = useActor('superStats');
  const {
    setHoldersData, setRewardPoolData, setReservePoolData, setTreasuryData, setSNSFundCanister,
  } = useCharts();

  const getHoldersData = async () => {
    try {
      const result = await superStats.get_activity_stats(projectStartDateDiffDays);
      setHoldersData(result);
      return result;
    } catch (err) {
      console.log('Failed to fetch `get_activity_stats for superstats`', err);
      return false;
    }
  };

  const getRewardPool = async () => {
    try {
      const result = await superStats.get_account_history({
        days: BigInt(projectStartDateDiffDays),
        account: 'iyehc-lqaaa-aaaap-ab25a-cai.0000000000000000000000000000000000000000000000000000000000000000',
      });
      setRewardPoolData(result);
      return result;
    } catch (e) {
      console.log("failed to fetch 'get_account_history'", e);
      return false;
    }
  };

  const getReservePool = async () => {
    try {
      const result = await superStats.get_account_history({
        days: BigInt(projectStartDateDiffDays),
        account: 'iyehc-lqaaa-aaaap-ab25a-cai.0100000000000000000000000000000000000000000000000000000000000000',
      });
      setReservePoolData(result);
      return result;
    } catch (e) {
      console.log("failed to fetch 'get_account_history'", e);
      return false;
    }
  };

  const getTreasuryData = async () => {
    try {
      const result = await superStats.get_account_history({
        days: BigInt(projectStartDateDiffDays),
        account: 'tr3th-kiaaa-aaaaq-aab6q-cai-nif4qry.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce',
      });
      setTreasuryData(result);
      return result;
    } catch (e) {
      console.log("failed to fetch 'get_account_history'", e);
      return false;
    }
  };

  const getSNSFundCanister = async () => {
    try {
      const result = await superStats.get_principal_history({
        days: BigInt(projectStartDateDiffDays),
        account: 't7z6p-ryaaa-aaaaq-aab7q-cai',
      });
      setSNSFundCanister(result);
      return result;
    } catch (e) {
      console.log("failed to fetch 'get_account_history'", e);
      return false;
    }
  };

  return {
    getHoldersData, getRewardPool, getReservePool, getTreasuryData, getSNSFundCanister,
  };
};

export default useSuperStats;
