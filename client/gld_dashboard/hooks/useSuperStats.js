import useActor from './useActor';
import useCharts from './useCharts';

const useSuperStats = () => {
  const [superStats] = useActor('superStats');
  const { setHoldersData } = useCharts();

  const getHoldersData = async () => {
    try {
      const result = await superStats.get_activity_stats(365);
      setHoldersData(result);
      return result;
    } catch (err) {
      console.log('Failed to fetch `get_activity_stats for superstats`', err);
      return false;
    }
  };

  return { getHoldersData };
};

export default useSuperStats;
