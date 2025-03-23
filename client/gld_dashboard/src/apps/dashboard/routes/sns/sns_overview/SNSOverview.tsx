import { ReactNode } from "react";

import { useAuth } from "@auth/index";
import { SNS_GOVERNANCE_CANISTER_ID_IC } from "@constants";

import { LoaderSpin } from "@components/index";

import useFetchNervousSystemParameters from "@services/sns_governance/hooks/useFetchNervousSystemParameters";

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
        <div className="font-semibold text-sm text-content/60">{title}</div>

        <div className="mt-3 font-semibold">{children}</div>
      </div>
    </div>
  );
};

const SNSOverview = () => {
  const { unauthenticatedAgent } = useAuth();
  const { data, isLoading, isSuccess, isError } =
    useFetchNervousSystemParameters(
      SNS_GOVERNANCE_CANISTER_ID_IC,
      unauthenticatedAgent,
      {
        enabled: !!unauthenticatedAgent,
      }
    );

  return (
    <>
      {isSuccess && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
          <Card title="Status">
            <div
              className={`rounded-full inline-flex items-center justify-center gap-2 bg-[#d9f2e8] py-2 px-4`}
            >
              <div className={`text-xs font-semibold shrink-0 text-black`}>
                {data.status}
              </div>
            </div>
          </Card>
          <Card title="Token Name">{data.token_name}</Card>
          <Card title="Token Symbol">{data.token_symbol}</Card>
          <Card title="Transaction Fee">
            {data.transaction_fee} <span>GLDGov</span>
          </Card>

          <Card title="Initial Voting Period">
            {data.initial_voting_period}
          </Card>

          <Card title="Max Voting Period Extension">
            {data.neuron_minimum_dissolve_delay_to_vote}
          </Card>

          <Card title="Reject Cost">
            {data.reject_cost} <span>GLDGov</span>
          </Card>

          <Card title="Min Neuron Stake">
            {data.neuron_minimum_stake} <span>GLDGov</span>
          </Card>

          <Card title="Min Dissolve Delay to Vote">
            {data.neuron_minimum_dissolve_delay_to_vote}
          </Card>

          <Card title="Max Dissolve Delay">{data.max_dissolve_delay}</Card>

          <Card title="Max Dissolve Delay Bonus">
            {data.max_dissolve_delay_bonus_percentage}%
          </Card>

          <Card title="Max Age for Age Bonus">
            {data.max_neuron_age_for_age_bonus}
          </Card>

          <Card title="Max Age Bonus">{data.max_age_bonus_percentage}</Card>

          <Card title="Reward Rate">
            {data.initial_reward_rate_basis_points}%
          </Card>
        </div>
      )}
      {(isLoading || isError) && (
        <div className="flex justify-center py-8">
          <LoaderSpin />
        </div>
      )}
    </>
  );
};

export default SNSOverview;
