'use client';

import { useEffect, useState } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { Bounce, toast } from 'react-toastify';
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
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
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
        To successfully connect each neuron to the dashboard, you need to
        add your principal  <span className="font-bold inline">
          {principal}
          <Image
            src={'/svg/copy-button.svg'}
            alt="copy"
            className="ml-2 cursor-pointer inline"
            height={15}
            width={15}
            onClick={() => copyContent(principal, setCopyState)}
          />
          </span> as a hotkey to your Gold DAO neuron. To do so, open the <Link
            href={'https://nns.ic0.app/neurons/?u=tw2vt-hqaaa-aaaaq-aab6a-cai'}
            target="_blank"
            rel="noreferrer noopener"
            className="underline "
          >
            NNS dApp
          </Link>, click into each neuron and add your principal as a hotkey.
        </p>
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
