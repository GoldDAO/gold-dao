import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/gld_dashboard_maintenance_mode/idlFactory";
import get_gld_dashboard_maintenance_mode from "@services/gld_dashboard_maintenance_mode/get_gld_dashboard_maintenance_mode";

const useGetGLDDashboardMaintenanceMode = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<boolean>, "queryKey" | "queryFn"> = {}
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
  } = options;

  return useQuery({
    queryKey: ["GET_GLD_DASHBOARD_MAINTENANCE_MODE", canisterId],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });
        const result = await get_gld_dashboard_maintenance_mode(actor);
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch GLD dashboard maintenance mode error! Please retry later."
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useGetGLDDashboardMaintenanceMode;
