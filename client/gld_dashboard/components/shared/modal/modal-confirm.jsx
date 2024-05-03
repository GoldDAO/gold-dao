'use client';

import { useEffect } from 'react';
import { truncatePrincipal } from '../../../utils/functions';
import useNeurons from '../../../hooks/useNeurons';
import { useSession } from '../../../hooks/useSession';
import useBalances from '../../../hooks/useBalances';

export default function ModalConfirm({
  name, amount, claim, setNeuronModify, setGold, setIcp,
}) {
  const { principal } = useSession();
  const { getBalance } = useBalances();
  const { claimReward, loading, requestSent } = useNeurons({
    neuronId: name,
    token: claim,
  });

  useEffect(() => {
    if (requestSent && !loading) document.getElementById('my_modal_confirm').close();
  }, [requestSent, loading]);

  const handleConfirmClaim = async () => {
    await claimReward();
    setNeuronModify((prev) => !prev);
    const amountIcp = await getBalance();
    setIcp({ loading: false, amount: amountIcp });
    const newAmount = await getBalance('ledger');
    setGold({ loading: false, amount: newAmount });
  };
  return (
    <>
      <div className=" flex-col mt-6 width-[90%] h-60 flex justify-around items-center">
        <p className="text-center font-medium text-lg">
          You are about to claim{' '}
          <span className="font-bold">
            {amount / 1e8} {claim}.
          </span>
        </p>
        <p className="text-center font-medium text-lg flex flex-col">
          The rewards will be sent to your account{' '}
          <span className="font-bold">{truncatePrincipal(principal)}</span>
        </p>
        <button
          className={`px-10 py-4 rounded-3xl bg-[#D3B871] text-white text-md font-bold flex items-center justify-center ${loading ? 'opacity-35 gap-2' : ''}`}
          onClick={handleConfirmClaim}
          disabled={loading}
        >
          {loading && <span className="loading loading-spinner"></span>}
          Confirm
        </button>
      </div>
    </>
  );
}
