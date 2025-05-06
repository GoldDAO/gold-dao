import { ReactNode } from "react";
import { useParams } from "react-router-dom";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";

import { LoaderSpin } from "@components/index";
import { BadgeProposalStatus } from "@components/badges/BadgeProposalStatus";

import useFetchOneProposal from "@services/sns_governance/hooks/useFetchOneProposal";

const CardSection = ({
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
        <div>
          <div className="font-semibold text-xl">{title}</div>
        </div>
        <div className="mt-6">{children}</div>
      </div>
    </div>
  );
};

const CardInner = ({
  title,
  children,
  className,
}: {
  title: string;
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div className={`border border-border bg-surface-primary/40 ${className}`}>
      <div className="px-6 pt-6 pb-8 text-center md:text-left">
        <div>
          <div className="font-semibold text-sm text-content/60">{title}</div>
        </div>
        <div className="mt-6">{children}</div>
      </div>
    </div>
  );
};

const GoldDAOProposalDetails = () => {
  const params = useParams();

  const { data, isLoading, isSuccess, isError } = useFetchOneProposal({
    proposalId: params.id as string,
  });

  console.log(data);

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Proposal</div>
      </div>

      <section className="pb-16">
        {isSuccess && (
          <div className="grid grid-cols-3 gap-4">
            <div className="col-span-3 xl:col-span-2">
              <CardSection title="Summary" className="mb-4">
                <Markdown
                  remarkPlugins={[remarkGfm]}
                  rehypePlugins={[rehypeRaw]}
                  components={{
                    hr: () => null,
                  }}
                >
                  {data.summary}
                </Markdown>
              </CardSection>

              <CardSection title="Payload" className="mb-4">
                <Markdown
                  remarkPlugins={[remarkGfm]}
                  rehypePlugins={[rehypeRaw]}
                  className="whitespace-pre-wrap break-all"
                >
                  {data.payload}
                </Markdown>
              </CardSection>

              <div>
                <div className="border border-border bg-surface-primary/40 rounded-xl">
                  <div className="px-6 pt-6 pb-8 text-center md:text-left font-semibold text-xl">
                    Overview
                  </div>
                  <div className="grid grid-cols-1 xl:grid-cols-2">
                    <CardInner title="Type" className="">
                      {data.type}
                    </CardInner>
                    <CardInner title="Status" className="">
                      <div className="inline-flex">
                        <BadgeProposalStatus
                          status={data.status as "open" | "executed"}
                        />
                      </div>
                    </CardInner>
                    <CardInner title="Type" className="">
                      {data.type}
                    </CardInner>
                    <CardInner title="Type" className="">
                      {data.type}
                    </CardInner>
                    <CardInner title="Type" className="">
                      {data.type}
                    </CardInner>
                    <CardInner title="Type" className="">
                      {data.type}
                    </CardInner>
                    <CardInner
                      title="Type"
                      className="col-span-1 xl:col-span-2"
                    >
                      {data.type}
                    </CardInner>
                    <CardInner
                      title="Type"
                      className="col-span-1 xl:col-span-2"
                    >
                      {data.type}
                    </CardInner>
                  </div>
                </div>
              </div>
            </div>
            <div className="order-first xl:order-last col-span-3 xl:col-span-1">
              Voting Results
            </div>
          </div>
          // <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 xl:grid-cols-4 gap-2">

          //   <Card title="Staked Amount">{data.staked_amount}</Card>
          //   <Card title="Total Maturity">{data.total_maturity}</Card>
          //   <Card title="Staked Maturity">{data.staked_maturity}</Card>

          //   <Card title="Dissolve Delay">{data.dissolve_delay}</Card>

          //   <Card title="Age">{data.age}</Card>
          //   <Card title="Date Created">{data.created_at}</Card>
          //   <Card title="Max Neuron Age For Age Bonus">
          //     {data.max_neuron_age_for_age_bonus}
          //   </Card>
          //   <Card title="Max Age Bonus Percentage">
          //     {data.max_age_bonus_percentage}
          //   </Card>
          //   <Card title="Dissolve Delay Bonus">
          //     {data.dissolve_delay_bonus}
          //   </Card>
          //   <Card title="Age Bonus">{data.age_bonus}</Card>
          //   <Card title="Total Bonus">{data.total_bonus}</Card>

          //   <Card title="Auto-Stake Maturity">{data.auto_stake_maturity}</Card>
          //   <Card title="Voting Power">{data.voting_power}</Card>
          //   <Card className="col-span-1 xl:col-span-2" title="Vesting Period">
          //     data.vestingPeriod
          //   </Card>
          // </div>
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

export default GoldDAOProposalDetails;
