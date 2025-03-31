import { ReactNode } from "react";
import { useParams } from "react-router-dom";

import { LoaderSpin } from "@components/index";
import {
  BadgeNeuronState,
  NeuronState,
} from "@components/badges/BadgeNeuronState";

import { useAuth } from "@auth/index";
import {
  SNS_GOVERNANCE_CANISTER_ID_IC,
  SNS_ROOT_CANISTER_ID_IC,
} from "@constants";

import useFetchOneNeuron from "@services/sns_governance/hooks/useFetchOneNeuron";

const Card = ({
  title,
  children,
  className,
}: {
  title: string;
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div
      className={`border border-border bg-surface-primary/40 rounded-xl ${className}`}
    >
      <div className="px-6 pt-6 pb-8 text-center md:text-left">
        <div className="">
          <div className="font-semibold text-sm">{title}</div>
        </div>
        <div className="mt-3">{children}</div>
      </div>
    </div>
  );
};

const GoldDAONeuronDetails = () => {
  const { unauthenticatedAgent } = useAuth();
  const params = useParams();

  const { data, isLoading, isSuccess, isError } = useFetchOneNeuron(
    SNS_GOVERNANCE_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      neuronId: params.id as string,
      snsRootCanisterId: SNS_ROOT_CANISTER_ID_IC,
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Neuron</div>
      </div>

      <section className="pb-16">
        {isSuccess && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
            <Card title="State">
              <BadgeNeuronState
                className="inline-block"
                state={data.state as NeuronState}
              />
            </Card>
            <Card title="Staked Amount">{data.staked_amount}</Card>
            <Card title="Total Maturity">{data.total_maturity}</Card>
            <Card title="Staked Maturity">{data.staked_maturity}</Card>

            <Card title="Dissolve Delay">{data.dissolve_delay}</Card>

            <Card title="Age">{data.age}</Card>
            <Card title="Date Created">{data.created_at}</Card>
            <Card title="Max Neuron Age For Age Bonus">
              {data.max_neuron_age_for_age_bonus}
            </Card>
            <Card title="Max Age Bonus Percentage">
              {data.max_age_bonus_percentage}
            </Card>
            <Card title="Dissolve Delay Bonus">
              {data.dissolve_delay_bonus}
            </Card>
            <Card title="Age Bonus">{data.age_bonus}</Card>
            <Card title="Total Bonus">{data.total_bonus}</Card>

            <Card title="Auto-Stake Maturity">{data.auto_stake_maturity}</Card>
            <Card title="Voting Power">{data.voting_power}</Card>
            <Card className="col-span-1 xl:col-span-2" title="Vesting Period">
              data.vestingPeriod
            </Card>
          </div>
        )}
        {(isLoading || isError) && (
          <div className="flex justify-center py-8">
            <LoaderSpin />
          </div>
        )}
      </section>
    </>
  );
};

export default GoldDAONeuronDetails;
