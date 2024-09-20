import useActor from './useActor';
import useCharts from './useCharts';

const useTokenMetrics = () => {
  const [tokenMetrics] = useActor('tokenMetrics');
  const { setStakersMetrics } = useCharts();

  const getStakedAmount = async () => {
    try {
      const result = await tokenMetrics.get_stake_history(365);
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
