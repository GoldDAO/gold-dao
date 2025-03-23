import { ActorSubclass } from "@dfinity/agent";
import { NervousSystemParameters } from "./interfaces";
import { parseNervousSystemParameters } from "./utils/parseNervousSystemParameters";
import { NervousSystemParameter } from "./utils/interfaces";

const get_nervous_system_parameters = async ({
  actor,
}: {
  actor: ActorSubclass;
}): Promise<NervousSystemParameter> => {
  const result = (await actor.get_nervous_system_parameters(
    null
  )) as NervousSystemParameters;

  const data = parseNervousSystemParameters(result);
  return data;
};

export default get_nervous_system_parameters;
