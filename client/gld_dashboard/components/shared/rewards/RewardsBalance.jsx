import { useEffect, useState } from 'react';

import LoaderDataCard from '../../Home/LoaderDataCard';
import Modal from '../modal/modal';
import ModalTransfer from '../modal/modal-transfer';
import RewardsCards from './RewardsCards';
import useBalances from '../../../hooks/useBalances';

export default function RewardsBalance({
  setOgy, setIcp, setGold, ogy, icp, gold,
}) {
  const [modalTitle, setModaltitle] = useState('');
  const [amount, setAmount] = useState(0);

  const { getBalance } = useBalances();

  useEffect(() => {
    // icp balance
    (async () => {
      const balance = await getBalance();
      setIcp({ loading: false, amount: balance });
    })();
    // ogy balance
    (async () => {
      const balance = await getBalance('ogy');
      setOgy({ loading: false, amount: balance });
    })();
    // gold
    (async () => {
      const balance = await getBalance('ledger');
      setGold({ loading: false, amount: balance });
    })();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <section className="grid grid-cols-1 md:grid-cols-1 lg:grid-cols-3 sm:grid-cols-3 gap-2 w-[100%] my-0 sm:px-2">
      {icp.loading ? (
        <LoaderDataCard className="basis-1/3" />
      ) : (
        <RewardsCards
          title="ICP"
          value={icp.amount}
          svg="/png/dfinity.png"
          status={true}
          setModalTitle={setModaltitle}
          setAmount={setAmount}
          loading={icp.loading}
          setIcp={setIcp}
        />
      )}
      {gold.loading ? (
        <LoaderDataCard className="basis-1/3" />
      ) : (
        <RewardsCards
          title="GLDGov"
          value={gold.amount}
          svg="svg/g-logo.svg"
          status={true}
          setModalTitle={setModaltitle}
          setAmount={setAmount}
          loading={gold.loading}
          setGold={setGold}
        />
      )}
      {ogy.loading ? (
        <LoaderDataCard className="basis-1/3" />
      ) : (
        <RewardsCards
          title="OGY"
          value={ogy.amount}
          svg="/png/origyn.png"
          status={true}
          setModalTitle={setModaltitle}
          setAmount={setAmount}
          loading={ogy.loading}
          setOgy={setOgy}
        />
      )}
      <Modal title="Transfer" idModal="my_modal_1">
        <ModalTransfer
          title={modalTitle}
          amount={amount}
          setGold={setGold}
          setIcp={setIcp}
          setOgy={setOgy}
          setAmount={setAmount}
        />
      </Modal>
    </section>
  );
}
