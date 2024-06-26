'use client';

import { useEffect } from 'react';
import { truncateNeuronId } from '../../../utils/functions';
import useNeurons from '../../../hooks/useNeurons';
import useSession from '../../../hooks/useSession';

export default function ModalDelete({ neuronId, setNeuronModify }) {
  const { removeNeuron, loading, requestSent } = useNeurons({
    neuronId,
    token: '',
  });
  useEffect(() => {
    if (requestSent && !loading) document.getElementById('my_modal_delete').close();
  }, [requestSent, loading]);
  const { principal } = useSession();

  const handleRemove = async () => {
    await removeNeuron();
    setNeuronModify((prev) => !prev);
  };

  return (
    <>
      <div className=" flex-col mt-6 width-[90%] h-60 flex justify-around items-center">
        <p className="text-center font-medium text-lg">
          You are about to remove the neuron{' '}
          <span className="font-bold">{`${truncateNeuronId(neuronId)}`}</span> from your dashbaord.
          Make sure to also remove your principal <span className="font-bold">{principal}</span>{' '}
          from the list of hotkeys of your neuron.
        </p>
        <button
          className={`mt-4 px-10 py-4 rounded-3xl bg-[#D3B871] text-white text-md font-bold flex items-center justify-center ${loading ? 'opacity-35 gap-2' : ''}`}
          onClick={handleRemove}
          disabled={loading}
        >
          {loading && <span className="loading loading-spinner"></span>}
          Confirm
        </button>
      </div>
    </>
  );
}
