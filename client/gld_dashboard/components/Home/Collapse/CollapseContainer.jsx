'use client';

/* eslint-disable no-nested-ternary */
import { useEffect, useState } from 'react';

import CanistersHeader from './Headers/CanistersHeader';
import CanistersItem from '../Items/CanistersItem';
import Collapse from './Collapse';
import NeuronsHeader from './Headers/NeuronsHeader';
import NeuronsItem from '../Items/NeuronsItem';
import OverviewItem from '../Items/OverviewItem';
import ProposalItem from '../Items/ProposalItem';
import ProposalsHeader from './Headers/ProposalsHeader';
import TransactionItem from '../Items/TransactionItem';
import TransactionsHeader from './Headers/TransactionsHeader';
import useServices from '../../../hooks/useServices';

const CollapseContainer = ({ className }) => {
  const {
    getProposals, getTxs, overviewData, goldNeurons, getCanisters,
  } = useServices();
  const [proposals, setProposals] = useState({ loading: true, data: [] });
  const [overview, setOverview] = useState({ loading: true, data: [] });
  const [transactions, setTransactions] = useState({ loading: true, data: [] });
  const [canisters, setCanisters] = useState({ loading: true, data: [] });
  const [neurons, setNeurons] = useState({ loading: true, data: [] });

  // eslint-disable-next-line no-unused-vars
  const [infoModal, setInfoModal] = useState(null);
  useEffect(() => {
    // proposals
    (async () => {
      const data = await getProposals({ limit: 5 });
      setProposals({ loading: false, data });
    })();
    // overview
    (async () => {
      const data = await overviewData();
      setOverview({ loading: false, data });
    })();
    // transactions
    (async () => {
      const data = await getTxs();
      setTransactions({ loading: false, data });
    })();
    // canisters
    (async () => {
      const data = await getCanisters();
      setCanisters({ loading: false, data });
    })();
    // neurons
    (async () => {
      const data = await goldNeurons();
      setNeurons({ loading: false, data });
    })();
  }, []);

  return (
    <section className={`flex flex-col gap-4 mt-4 ${className}`}>
      <Collapse
        className='hidden sm:block'
        title="Overview"
        setInfoModal={setInfoModal}
        item={overview} name={'overview'}
        redirect='overview'
        key={'overview'}
      >
        {overview.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading overview...
            </section>
          </article>
        ) : overview?.data?.length > 0 ? (
          overview?.data?.map((c) => <OverviewItem key={`overview-${c.id}`} {...c} />)
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch overview data. Please, retry again.
            </section>
          </article>
        )}
      </Collapse>
      <Collapse title="Proposals" setInfoModal={setInfoModal} item={proposals} name={'proposals'} redirect='proposals' key={'proposals'}>
        <ProposalsHeader />
        <div className="divide-y">
          {proposals.loading ? (
            <article className="collapse w-full border-[1px] rounded-none flex justify-center">
              <section className="collapse-title flex gap-2 justify-center items-center h-20">
                <span className="loading loading-spinner"></span>
                Loading proposals...
              </section>
            </article>
          ) : proposals?.data?.length > 0 ? (
            proposals?.data?.map((c) => <ProposalItem key={c.id} {...c} />)
          ) : (
            <article className="collapse w-full border-[1px] rounded-none flex justify-center">
              <section className="collapse-title flex gap-2 justify-center items-center h-20">
                Fail to fetch proposals data. Please, retry again.
              </section>
            </article>
          )}
        </div>
      </Collapse>
      <Collapse
        title="GLDGov Transactions"
        setInfoModal={setInfoModal}
        item={transactions}
        name={'transactions'}
        redirect='transactions'
        key={'transactions'}
      >
        <TransactionsHeader />
        {transactions.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading transactions...
            </section>
          </article>
        ) : transactions?.data?.length > 0 ? (
          transactions?.data?.map((c) => <TransactionItem key={`transaction-${c.id}`} {...c} />)
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch transactions data. Please, retry again.
            </section>
          </article>
        )}
      </Collapse>
      <Collapse
        title="Gold DAO Canisters"
        setInfoModal={setInfoModal}
        item={canisters}
        name={'canisters'}
        redirect='canisters'
        key={'canisters'}
      >
        <CanistersHeader />
        {canisters.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading canisters...
            </section>
          </article>
        ) : canisters?.data?.length > 0 ? (
          canisters?.data?.map((c) => <CanistersItem key={c.id} {...c} />)
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch canisters data. Please, retry again.
            </section>
          </article>
        )}
      </Collapse>
      <Collapse
        title="Gold DAO Neurons"
        setInfoModal={setInfoModal}
        item={neurons}
        name={'neurons'}
        redirect='neurons'
        key={'neurons'}
      >
        <NeuronsHeader />
        {neurons.loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading neurons...
            </section>
          </article>
        ) : neurons?.data?.length > 0 ? (
          neurons?.data?.map((c) => <NeuronsItem key={c.id} neuron={c} />)
        ) : (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              Fail to fetch neurons data. Please, retry again.
            </section>
          </article>
        )}
      </Collapse>
    </section>
  );
};

export default CollapseContainer;
