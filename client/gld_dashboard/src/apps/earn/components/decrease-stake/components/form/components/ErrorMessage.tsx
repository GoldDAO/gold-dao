import { useAtom } from "jotai";
import { MAX_DISSOLVE_EVENTS } from "@constants";
import { DecreaseStakeStateReducerAtom } from "@earn/components/decrease-stake/atoms";

const ErrorMessage = () => {
  const [state] = useAtom(DecreaseStakeStateReducerAtom);

  if (state.user_staked_data.remaining_dissolve_events === 0)
    return `No remaining dissolve positions (Max. ${MAX_DISSOLVE_EVENTS})`;

  return "Next";
};

export default ErrorMessage;
