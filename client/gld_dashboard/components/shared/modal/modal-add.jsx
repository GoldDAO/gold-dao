'use client';

import { useEffect, useState } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { copyContent } from '../../../utils/functions';
import useNeurons from '../../../hooks/useNeurons';
import useSession from '../../../hooks/useSession';

export default function ModalAdd({ setNeuronModify, neuronModify }) {
  const [neuronIdToAdd, setNeuronIdToAdd] = useState('');
  const [copyState, setCopyState] = useState(false);
  const { principal } = useSession();

  const { loading, requestSent } = useNeurons({
    neuronId: neuronIdToAdd,
    token: '',
    neuronsToClaim: [],
  });

  useEffect(() => {
    if (requestSent && !loading) setNeuronIdToAdd('');
  }, [requestSent, loading]);

  useEffect(() => {
    if (copyState) {
      setTimeout(() => {
        setCopyState(false);
      }, 2000);
    }
  }, [copyState]);

  const closeModal = async () => {
    document.getElementById('my_modal_add')?.close();
    setNeuronModify(neuronModify + 1);
  };

  return (
    <div className="flex-col mt-6 mb-32 overflow-y-auto sm:mb-0 flex justify-around items-center">
      <div className="text-left font-normal text-lg w-full flex flex-wrap gap-[36px]">
        <p>
          To successfully add each neuron to the dashboard,
          please complete the following for every individual neuron:
        </p>
        <span className="flex flex-wrap">
          {'1. Add your principal'.split(' ').map((l, i) => (
            <span className="mr-1" key={i * 999}>
              {l}
            </span>
          ))}
          {principal.split('').map((l, i) => (
            <span className="font-bold" key={i * 22}>
              {l}
            </span>
          ))}
          <Image
            src={'/svg/copy-button.svg'}
            alt="copy"
            className="mx-2 cursor-pointer"
            height={15}
            width={15}
            onClick={() => copyContent(principal, setCopyState)}
          />
          <span className="mr-1">as</span>
          <span className="font-bold mr-1">a</span>
          <span className="font-bold mr-1">HotKey</span>
          <span className="mr-1">to</span>
          <span className="mr-1">your</span>
          <span className="font-bold mr-1">Gold</span>
          <span className="font-bold mr-1">DAO</span>
          <span className="font-bold mr-1">neuron</span>
          <span className="mr-1">which</span>
          <span className="mr-1">you</span>
          <span className="mr-1">wish</span>
          <span className="mr-1">to</span>
          <span className="mr-1">include</span>
          <span className="mr-1">in</span>
          <span className="mr-1">this</span>
          <span className="mr-1">dashbaord.</span>
          <span className="mr-1">To</span>
          <span className="mr-1">do</span>
          <span className="mr-1">this,</span>
          <span className="mr-1">Open</span>
          <span className="mr-1">your</span>
          <Link
            href={'https://nns.ic0.app/neurons/?u=tw2vt-hqaaa-aaaaq-aab6a-cai'}
            target="_blank"
            rel="noreferrer noopener"
            className="underline mr-1"
          >
            NNS app
          </Link>
          <span className="mr-1">and</span>
          <span className="mr-1">click</span>
          <span className="mr-1">into</span>
          <span className="mr-1">each</span>
          <span className="mr-1">Neuron.</span>
        </span>
        {copyState && (
          <div className="text-green-600">
            <p>copied</p>
          </div>
        )}
      </div>

      <button
        className={`px-10 mt-6 py-4 rounded-3xl bg-[#D3B871] text-white text-md font-bold flex items-center justify-center ${loading ? 'opacity-35 gap-2' : ''} disabled:opacity-35`}
        onClick={closeModal}
      >
        {loading && <span className="loading loading-spinner"></span>}
        Done
      </button>
    </div>
  );
}
