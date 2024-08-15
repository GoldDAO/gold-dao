'use client';

import { useEffect, useState } from 'react';
import DataCard from './DataCard';
import Modal from '../shared/modal/modal';
import ModalNeuron from '../shared/modal/modal-neuron';
import useServices from '../../hooks/useServices';

export default function Neurons() {
  const [dataNeuron, setDataNeuron] = useState(null);
  const { icpNeurons, ogyNeurons } = useServices();
  const [data, setData] = useState([
    {
      title: 'ICP',
      amount: 0,
      image: 'png/dfinity.png',
      info: 'Total amount of ICP stake in neurons owned by the Gold DAO.',
    },
    {
      title: 'OGY',
      amount: 0,
      image: 'png/origyn.png',
      info: 'Total amount of OGY stake in a neuron owned by the Gold DAO. ',
    },
  ]);
  const [neurons, setNeurons] = useState({ OGY: [], ICP: [] });

  useEffect(() => {
    (async () => {
      const icpData = await icpNeurons();
      const ogyData = await ogyNeurons();
      const tmpData = data;
      tmpData[0].amount = icpData.reduce((res, item) => res + item.stakedAmount, 0);
      tmpData[1].amount = ogyData.reduce((res, item) => res + item.stakedAmount, 0);
      setData(tmpData);
      setNeurons({
        ICP: icpData,
        OGY: ogyData,
      });
    })();
  }, []);

  return (
    <section className="block sm:flex w-full mt-5 gap-2">
      {data.map((d) => (
        <DataCard key={d.title} {...d} className="basis-1/2" setDataNeuron={setDataNeuron} />
      ))}
      <Modal title={`${dataNeuron?.name} Neurons`} idModal="neuronmodal">
        <ModalNeuron {...dataNeuron} neurons={neurons} />
      </Modal>
    </section>
  );
}
