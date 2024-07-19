'use client';

import { useEffect, useState } from 'react';
import { data1 } from '../../utils/datas';

import DataCardCopy from './DataCardCopy';
import LoaderDataCard from './LoaderDataCard';
import Modal from '../shared/modal/modal';
import ModalChart from '../shared/modal/modal-chart';
import useServices from '../../hooks/useServices';
import ModalChartMobile from '../shared/modal/modal-chart-mobile';

export default function GLDGovInfo() {
  const { gldGovTotalSupply, gldGovPrice } = useServices();
  const [totalSupply, setTotalSupply] = useState({ loading: true, amount: 0 });
  const [price, setPrice] = useState({ loading: true, amount: 0 });
  const [marketcap, setMarketcap] = useState({ loading: true, amount: 0 });
  const [infoModal, setInfoModal] = useState(null);
  // getters
  useEffect(() => {
    (async () => {
      const amount = await gldGovTotalSupply();
      setTotalSupply({ loading: false, amount });
    })();
    (async () => {
      const amount = await gldGovPrice();
      setPrice({ loading: false, amount });
    })();
  }, []);

  // setter mktcap
  useEffect(() => {
    if (!totalSupply.loading && !price.loading) {
      setMarketcap({
        loading: false,
        amount: parseFloat(totalSupply.amount) * parseFloat(price.amount),
      });
    }
  }, [totalSupply, price]);

  const allDataLoaded = !totalSupply.loading && !price.loading && !marketcap.loading;

  return (
    <>
      <section className="grid grid-cols-1 lg:grid-cols-3 w-full gap-2">
        {allDataLoaded ? (
          <>
            <DataCardCopy
              title="Total GLDGov Supply"
              image="svg/g-logo.svg"
              info="Total amount of GLDGov tokens in existence.."
              amount={totalSupply.amount}
              className="basis-1/3"
              setInfoModal={setInfoModal}
              data={data1}
              openModal={false}
            />
            <DataCardCopy
              title="GLDGov Price"
              isPrice={true}
              info="Average price of GLDGov on the market."
              amount={price.amount}
              className="basis-1/3"
              setInfoModal={setInfoModal}
              data={data1}
              openModal={false}
            />
            <DataCardCopy
              title="GLDGov Marketcap"
              isPrice={true}
              info="Total amount of GLDGov multiplied by the average price."
              amount={marketcap.amount}
              className="basis-1/3"
              setInfoModal={setInfoModal}
              data={data1}
              openModal={false}
            />
          </>
        ) : (
          <>
            <LoaderDataCard className="basis-1/3" />
            <LoaderDataCard className="basis-1/3" />
            <LoaderDataCard className="basis-1/3" />
          </>
        )}
      </section>
      <Modal
        title={`chart ${infoModal?.title}`}
        idModal="chartmodal"
        amount={infoModal?.amount}
      >
        <ModalChart name={infoModal?.title} />
      </Modal>
      <Modal
        title={`chart ${infoModal?.title}`}
        idModal="chartmodalheader"
        amount={infoModal?.amount}
      >
        <ModalChartMobile name={infoModal?.title} />
      </Modal>
    </>
  );
}
