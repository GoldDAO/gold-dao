import useActor from './useActor';

const useManagement = () => {
  const [management] = useActor('management');

  const getMaintenanceMode = async () => {
    try {
      const result = await management.get_gld_dashboard_maintenance_mode();

      return result;
    } catch (err) {
      console.log('Failed to fetch `get_gld_dashboard_maintenance_mode`');
      return false;
    }
  };

  return { getMaintenanceMode };
};

export default useManagement;
