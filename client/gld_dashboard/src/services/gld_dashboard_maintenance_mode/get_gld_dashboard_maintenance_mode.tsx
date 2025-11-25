import { ActorSubclass } from "@dfinity/agent";

const get_gld_dashboard_maintenance_mode = async (
  actor: ActorSubclass
): Promise<boolean> => {
  const result = (await actor.get_gld_dashboard_maintenance_mode()) as boolean;
  return result;
};

export default get_gld_dashboard_maintenance_mode;
