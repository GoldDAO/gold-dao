import { Bounce, toast } from 'react-toastify';

import { useState } from 'react';
import { p } from '../utils/parsers';
import useActor from './useActor';
import useSession from './useSession';

const useTransfer = ({ selectedToken, amount, to }) => {
  const [loading, setLoading] = useState(false);
  const { principal, isConnected } = useSession();
  const [token] = useActor(selectedToken);

  const icrc1Transfer = async () => {
    if (!isConnected) {
      toast.error('you must be logged.', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      return console.log('you must be logged.');
    }
    if (!token) {
      toast.error('you must be logged!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      return console.log('selected token is wrong.');
    }
    setLoading(true);
    try {
      const balance = Number(await token.icrc1_balance_of({ owner: p(principal), subaccount: [] }));
      const fee = Number(await token.icrc1_fee());
      const decimals = Number(await token.icrc1_decimals());

      const parsedAmount = parseInt(amount * 10 ** decimals, 10);
      if (balance < parsedAmount + fee) {
        setLoading(false);

        return toast.error('Insufficient balance', {
          position: 'top-right',
          autoClose: 5000,
          hideProgressBar: false,
          closeOnClick: true,
          pauseOnHover: true,
          draggable: true,
          progress: undefined,
          theme: 'light',
          transition: Bounce,
        });
      }

      const tx = await token.icrc1_transfer({
        to: { owner: p(to), subaccount: [] },
        fee: [],
        memo: [],
        from_subaccount: [],
        created_at_time: [],
        amount: parsedAmount,
      });

      if (tx.Ok) {
        setLoading(false);
        return toast.success('Transaction successful!', {
          position: 'top-right',
          autoClose: 5000,
          hideProgressBar: false,
          closeOnClick: true,
          pauseOnHover: true,
          draggable: true,
          progress: undefined,
          theme: 'light',
          transition: Bounce,
        });
      }
      setLoading(false);
      return toast.error('unexpected error!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
    } catch (err) {
      setLoading(false);
      return toast.error('unexpected error!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
    }
  };

  return { icrc1Transfer, loading };
};

export default useTransfer;
