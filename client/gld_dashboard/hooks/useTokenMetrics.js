import useActor from './useActor';
import useCharts from './useCharts';

// Define the specific date
const startDate = new Date('2023-11-13'); // minus 1 month
const today = new Date();
const timeDifference = today - startDate;
const projectStartDateDiffDays = Math.floor(timeDifference / (1000 * 60 * 60 * 24));

const useTokenMetrics = () => {
  const [tokenMetrics] = useActor('tokenMetrics');
  const { setStakersMetrics } = useCharts();

  const getStakedAmount = async () => {
    try {
      const result = await tokenMetrics.get_stake_history(projectStartDateDiffDays);
      setStakersMetrics(result);
      return result;
    } catch (err) {
      console.log('Failed to fetch `get_staked_history for tokenMetrics`', err);
      return false;
    }
  };

  return { getStakedAmount };
};

export default useTokenMetrics;
