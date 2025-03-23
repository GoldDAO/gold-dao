import { useNavigate } from "react-router-dom";
import { usePagination } from "@utils/table/useTable";

import SNSOverview from "./sns_overview/SNSOverview";
import GoldDAORewards from "./gold_dao_rewards/GoldDAORewards";
import { GoldDAOCanistersTable } from "./gold_dao_canisters/table/GoldDAOCanistersTable";
import { GLDGovTransactionsTable } from "./gldgov_transactions/table/GLDGovTransactionsTable";
import { GoldDAOProposalsTable } from "./gold_dao_proposals/table/GoldDAOProposalsTable";
import { GoldDAONeuronsTable } from "./gold_dao_neurons/table/GoldDAONeuronsTable";

export const SNS = () => {
  const navigate = useNavigate();
  const [paginationCanisters] = usePagination({ pageIndex: 0, pageSize: 5 });
  const [paginationTransactions] = usePagination({ pageIndex: 0, pageSize: 5 });
  const [paginationProposals] = usePagination({ pageIndex: 0, pageSize: 5 });
  const [paginationNeurons] = usePagination({ pageIndex: 0, pageSize: 5 });

  const handleNavigate = (
    to: "canisters" | "transactions" | "proposals" | "neurons"
  ) => {
    navigate(to);
  };

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">SNS</div>
      </div>

      <section className="pb-16">
        <SNSOverview />
      </section>

      <section className="border border-border rounded-xl bg-surface-primary/40 p-6 mb-6">
        <div className="flex items-center justify-between mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">Gold DAO Rewards</h6>
        </div>
        <GoldDAORewards />
      </section>

      <section className="rounded-xl bg-surface-primary/40 mb-6">
        <div className="flex items-center justify-between px-6 pt-6 mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">Gold DAO Canisters</h6>
          <div
            className="font-semibold text-accent cursor-pointer text-sm"
            onClick={() => handleNavigate("canisters")}
          >
            View All Canisters
          </div>
        </div>
        <GoldDAOCanistersTable pagination={paginationCanisters} />
      </section>

      <section className="rounded-xl bg-surface-primary/40 mb-6">
        <div className="flex items-center justify-between px-6 pt-6 mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">GLDGov Transactions</h6>
          <div
            className="font-semibold text-accent cursor-pointer text-sm"
            onClick={() => handleNavigate("transactions")}
          >
            View All Transactions
          </div>
        </div>
        <GLDGovTransactionsTable pagination={paginationTransactions} />
      </section>

      <section className="rounded-xl bg-surface-primary/40 mb-6">
        <div className="flex items-center justify-between px-6 pt-6 mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">Gold DAO Proposals</h6>
          <div
            className="font-semibold text-accent cursor-pointer text-sm"
            onClick={() => handleNavigate("proposals")}
          >
            View All Proposals
          </div>
        </div>
        <GoldDAOProposalsTable pagination={paginationProposals} />
      </section>

      <section className="rounded-xl bg-surface-primary/40 mb-6">
        <div className="flex items-center justify-between px-6 pt-6 mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">Gold DAO Neurons</h6>
          <div
            className="font-semibold text-accent cursor-pointer text-sm"
            onClick={() => handleNavigate("neurons")}
          >
            View All Neurons
          </div>
        </div>
        <GoldDAONeuronsTable pagination={paginationNeurons} />
      </section>
    </>
  );
};
