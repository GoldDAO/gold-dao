import useGetTotalStaked from "../../hooks/useGetTotalStaked";
import useGetNeuronSize from "../../hooks/useGetNeuronSize";
import useGetNumberOfStakers from "../../hooks/useGetNumberOfStakers";
import useGetCurrentAPY from "../../hooks/useGetCurrentAPY";
import { getCanister } from "../../utils/getCanister";
import { getNeuron } from "../../utils/getNeuron";
import NumberToLocaleString from "../shared/NumberToLocaleString";
import Card from "../shared/ui/Card";

const Summary = ({ env }) => {
  const {
    GLDT_STAKE_CANISTER_ID,
    GLDT_LEDGER_CANISTER_ID,
    GOLDAO_LEDGER_CANISTER_ID,
  } = getCanister(env);

  const totalStaked = useGetTotalStaked(
    GLDT_STAKE_CANISTER_ID,
    GLDT_LEDGER_CANISTER_ID
  );

  const neuronSize = useGetNeuronSize(
    GLDT_STAKE_CANISTER_ID,
    GOLDAO_LEDGER_CANISTER_ID
  );
  const numberOfStakers = useGetNumberOfStakers(GLDT_STAKE_CANISTER_ID);

  const currentAPY = useGetCurrentAPY(GLDT_STAKE_CANISTER_ID);

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      <Card className="">
        <Card.Header>
          <Card.Title>Total staked</Card.Title>
          <Card.Link
            href={`https://ic.house/address/6uad6-fqaaa-aaaam-abovq-cai/${
              getCanister(env).GLDT_STAKE_CANISTER_ID
            }.0300000000000000000000000000000000000000000000000000000000000000`}
          />
        </Card.Header>
        <div>
          <div className="text-xl md:text-4xl font-light">
            <div className="flex items-baseline gap-1">
              {totalStaked.isSuccess ? (
                <NumberToLocaleString value={totalStaked.data} />
              ) : (
                <div className="animate-pulse">0</div>
              )}
              <div className="text-neutral-900/60 dark:text-neutral-50/60 text-base">
                GLDT
              </div>
            </div>
          </div>
        </div>
      </Card>
      <Card className="">
        <Card.Header>
          <Card.Title>Neuron size</Card.Title>
          <Card.Link
            href={`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/neuron/${
              getNeuron(env).GLDT_STAKE_SOURCE_NEURON_ID
            }`}
          />
        </Card.Header>
        <div>
          <div className="text-2xl md:text-4xl font-light">
            <div className="flex items-baseline gap-1">
              {neuronSize.isSuccess ? (
                <NumberToLocaleString value={neuronSize.data} />
              ) : (
                <div className="animate-pulse">0</div>
              )}
              <div className="text-neutral-900/60 dark:text-neutral-50/60 text-base">
                GOLDAO
              </div>
            </div>
          </div>
        </div>
      </Card>
      <Card className="">
        <Card.Header>
          <Card.Title>Number of stakers</Card.Title>
        </Card.Header>
        <div>
          <div className="text-2xl md:text-4xl font-light">
            {numberOfStakers.isSuccess ? (
              <>
                <NumberToLocaleString value={numberOfStakers.data} />
              </>
            ) : (
              <div className="animate-pulse">0</div>
            )}
          </div>
        </div>
      </Card>
      <Card className="">
        <Card.Header>
          <Card.Title>Current APY</Card.Title>
        </Card.Header>
        <div>
          <div className="text-2xl md:text-4xl font-light">
            <div className="flex items-baseline gap-1">
              {currentAPY.isSuccess ? (
                <NumberToLocaleString value={currentAPY.data} />
              ) : (
                <div className="animate-pulse">0</div>
              )}
              <div className="text-neutral-900/60 dark:text-neutral-50/60 text-base">
                %
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default Summary;
