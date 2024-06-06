'use client';

import { useEffect } from 'react';
import useBalances from '../../../hooks/useBalances';
import useNeurons from '../../../hooks/useNeurons';
import useSession from '../../../hooks/useSession';

export default function ModalClaimAll({
  neuronAmountsToClaim, setGold, setIcp, setNeuronModify,
}) {
  const { principal } = useSession();
  const { claimAllRewards, loading, requestSent } = useNeurons({
    neuronsToClaim: neuronAmountsToClaim.userNeurons,
  });
  const { getBalance } = useBalances();
  const handleConfirmClaimAll = async () => {
    await claimAllRewards();
    const amountIcp = await getBalance();
    setIcp({ loading: false, amount: amountIcp });
    const amount = await getBalance('ledger');
    setGold({ loading: false, amount });
  };

  useEffect(() => {
    if (requestSent && !loading) {
      document.getElementById('my_modal_claim_desk').close();
      setNeuronModify((prev) => !prev);
    }
  }, [requestSent, loading]);

  return (
    <>
      <div className=" flex-col mt-6 width-[90%] h-60 flex justify-around items-center">
        <p className="text-center font-regular text-base sm:font-medium sm:text-lg">
          You are about to claim{' '}
          <span className="font-bold">{neuronAmountsToClaim.icpAmount / 1e8 || 0} ICP</span> and{' '}
          <span className="font-bold"> {neuronAmountsToClaim.ledgerAmount / 1e8 || 0} GLDGov. </span>
        </p>
        <p className="text-center font-regular text-base sm:font-medium sm:text-lg">
          The rewards will be sent to your account
          <p className="font-bold">{principal}</p>
        </p>
        <button
          className={`px-10 py-4 rounded-full bg-[#D3B871] text-white text-md font-bold flex items-center justify-center h-10 w-full sm:w-fit sm:h-fit ${(!neuronAmountsToClaim.icpAmount && !neuronAmountsToClaim.ledgerAmount) || loading ? 'opacity-35 cursor-not-allowed' : ''}`}
          onClick={handleConfirmClaimAll}
          disabled={(!neuronAmountsToClaim.icpAmount && !neuronAmountsToClaim.ledgerAmount)
            || loading}
        >
          {loading && <span className="loading loading-spinner mr-2"></span>}
          Confirm
        </button>
      </div>
    </>
  );
}
