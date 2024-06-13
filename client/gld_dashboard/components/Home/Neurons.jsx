'use client';

import { useState } from 'react';
import DataCard from './DataCard';
import Modal from '../shared/modal/modal';
import ModalNeuron from '../shared/modal/modal-neuron';

export default function Neurons() {
  const [dataNeuron, setDataNeuron] = useState(null);
  const data = [
    {
      title: 'ICP Neuron',
      amount: 548602.537,
      image: 'png/dfinity.png',
      info: 'Total amount of ICP stake in neurons owned by the Gold DAO.',
    },
    {
      title: 'OGY Neuron',
      amount: 500000000,
      image: 'png/origyn.png',
      info: 'Total amount of OGY stake in a neuron owned by the Gold DAO. ',
    },
  ];

  return (
    <section className="block sm:flex w-full mt-5 gap-2">
      {data.map((d) => (
        <DataCard key={d.title} {...d} className="basis-1/2" setDataNeuron={setDataNeuron} />
      ))}
      <Modal title={`${dataNeuron?.name} Neurons`} idModal="neuronmodal">
        <ModalNeuron {...dataNeuron} />
      </Modal>
    </section>
  );
}
