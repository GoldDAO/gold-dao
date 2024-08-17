/* eslint-disable consistent-return */
/* eslint-disable no-await-in-loop */
import { Bounce, toast } from 'react-toastify';
import { useState } from 'react';
import { calculateVotingPower, neuronState, uint8ArrayToHexString } from '../utils/functions';
import mapResponseErrorCodeToFriendlyError from '../utils/errorMap';

import canisters from '../utils/canisters';
import { p } from '../utils/parsers';
import useActor from './useActor';

const useNeurons = ({ neuronId, token, neuronsToClaim }) => {
  const [snsRewards] = useActor('snsRewards');
  const [governance] = useActor('governance');
  const [ogy] = useActor('ogy');
  const [ledger] = useActor('ledger');
  const [icp] = useActor('icp');
  const [loading, setLoading] = useState(false);
  const [neuronError, setNeuronError] = useState({});
  const [requestSent, setRequestSent] = useState(false);

  function hexStringToUint8Array(hexString) {
    const bytes = [];
    for (let i = 0; i < hexString.length; i += 2) {
      bytes.push(parseInt(hexString.substr(i, 2), 16));
    }
    return new Uint8Array(bytes);
  }

  const addNeuron = async () => {
    try {
      setRequestSent(true);
      setLoading(true);
      const fixedNeuronId = hexStringToUint8Array(neuronId);
      const response = await snsRewards.add_neuron_ownership({ id: fixedNeuronId });
      if (response.Ok) {
        setLoading(false);
        toast.success('Neuron successfully added!', {
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
        return true;
      }
      setLoading(false);
      toast.error(
        mapResponseErrorCodeToFriendlyError(response),
        {
          position: 'top-right',
          autoClose: 10000,
          hideProgressBar: false,
          closeOnClick: false,
          pauseOnHover: true,
          draggable: false,
          progress: undefined,
          theme: 'light',
          transition: Bounce,
        },
      );
      return false;
    } catch (err) {
      setLoading(false);
      toast.error('Something went wrong', {
        position: 'top-right',
        autoClose: 10000,
        hideProgressBar: false,
        closeOnClick: false,
        pauseOnHover: true,
        draggable: false,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      return false;
    }
  };

  const removeNeuron = async () => {
    try {
      setRequestSent(true);
      setLoading(true);
      const response = await snsRewards.remove_neuron_ownership({ id: neuronId });
      if (response.Ok) {
        setLoading(false);
        toast.success('Neuron successfully removed!', {
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
      } else {
        setLoading(false);
        toast.error(
          mapResponseErrorCodeToFriendlyError(response),
          {
            position: 'top-right',
            autoClose: 10000,
            hideProgressBar: false,
            closeOnClick: false,
            pauseOnHover: true,
            draggable: false,
            progress: undefined,
            theme: 'light',
            transition: Bounce,
          },
        );
      }
      return { loading };
    } catch (err) {
      setLoading(false);
      toast.error('Something went wrong', {
        position: 'top-right',
        autoClose: 10000,
        hideProgressBar: false,
        closeOnClick: false,
        pauseOnHover: true,
        draggable: false,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
    }
  };

  // call this function in a loop for each neuron the user has to claim all.
  const claimOneReward = async (id, tok) => {
    try {
      const response = await snsRewards.claim_reward({ token: tok, neuron_id: { id } });
      return response;
    } catch (err) {
      console.log(err);
      setNeuronError({ ...neuronError, id });
      const hexNeuronId = uint8ArrayToHexString(id);
      toast.error(`Something went wrong claiming neuron ${hexNeuronId} with token ${tok}`, {
        position: 'top-right',
        autoClose: 7000,
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

  const claimReward = async () => {
    setRequestSent(true);
    setLoading(true);
    const response = await claimOneReward(neuronId, token);
    if (response.Ok) {
      setLoading(false);
      setRequestSent(false);
      toast.success('Reward successfully claimed!', {
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
      return true;
    }
    setNeuronError({ ...neuronError, neuronId });
    const hexNeuronId = uint8ArrayToHexString(neuronId);
    toast.error(
      `claiming neuron ${hexNeuronId} with token ${token}. ${mapResponseErrorCodeToFriendlyError(response)}`,
      {
        position: 'top-right',
        autoClose: 10000,
        hideProgressBar: false,
        closeOnClick: false,
        pauseOnHover: true,
        draggable: false,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      },
    );

    setLoading(false);
  };

  const nervousSystemParameters = async () => {
    try {
      const getNeuronsParameters = await governance.get_nervous_system_parameters();
      return getNeuronsParameters;
    } catch (error) {
      console.log('error nervour system parameters', error);
    }
  };

  const getNeuronsByOwner = async () => {
    try {
      setRequestSent(true);
      setLoading(true);
      const neurons = {};
      let neuronIds = await snsRewards.get_neurons_by_owner([]);
      const neuronsParameters = await nervousSystemParameters();
      if (neuronIds.length) {
        neuronIds = neuronIds.flat();
        const neuronPromises = [];
        for (let i = 0; i < neuronIds.length; i += 1) {
          const promise = governance.get_neuron({ neuron_id: [neuronIds[i]] });
          neuronPromises.push(promise);
        }

        const responses = await Promise.all(neuronPromises);
        
        const neuronsData = responses.map(async (status, i) => {
          const fixedNeuronIds = Array.from(neuronIds[i].id);
          const neuronAge = Math.round(new Date().getTime() / 1000)
            - Number(status.result[0].Neuron.aging_since_timestamp_seconds);
          const dissolveState = status.result[0].Neuron.dissolve_state[0];
          const { votingPower, dissolveDelay } = calculateVotingPower({
            cachedNeuronStakeE8s: Number(status.result[0].Neuron.cached_neuron_stake_e8s),
            stakedMaturiryE8sEquivalent: Number(
              status.result[0].Neuron.staked_maturity_e8s_equivalent[0] || 0,
            ),
            age: neuronAge,
            maxNeuronAgeForAgeBonus: Number(neuronsParameters.max_neuron_age_for_age_bonus[0]),
            maxAgeBonusPercentage: Number(neuronsParameters.max_age_bonus_percentage[0]),
            dissolveState,
            maxDissolveDelayBonusPercentage: Number(
              neuronsParameters.max_dissolve_delay_bonus_percentage[0],
            ),
            maxDissolveDelaySeconds: Number(neuronsParameters.max_dissolve_delay_seconds[0]),
          });

          let icpRewards;
          let ledgerRewards;
          let ogyRewards;
          if (status.result[0].Neuron) {
            neurons[fixedNeuronIds] = {
              ...status.result[0].Neuron,
              id: status.result[0].Neuron.id[0].id,
            };
            icpRewards = await icp.icrc1_balance_of({
              owner: p(canisters.snsRewards.canisterId),
              subaccount: [fixedNeuronIds],
            });
            ledgerRewards = await ledger.icrc1_balance_of({
              owner: p(canisters.snsRewards.canisterId),
              subaccount: [fixedNeuronIds],
            });
            ogyRewards = await ogy.icrc1_balance_of({
              owner: p(canisters.snsRewards.canisterId),
              subaccount: [fixedNeuronIds],
            });
            console.log(icpRewards, ledgerRewards, ogyRewards);
          } else {
            setLoading(false);
            toast.error('Something went wrong, please retry later.', {
              position: 'top-right',
              autoClose: 7000,
              hideProgressBar: false,
              closeOnClick: true,
              pauseOnHover: true,
              draggable: true,
              progress: undefined,
              theme: 'light',
              transition: Bounce,
            });
          }

          neurons[fixedNeuronIds] = {
            ...neurons[fixedNeuronIds],
            icpRewards: Number(icpRewards),
            ledgerRewards: Number(ledgerRewards),
            ogyRewards: Number(ogyRewards),
            dissolving: neuronState(dissolveState),
            votingPower: dissolveDelay
              > Number(neuronsParameters.neuron_minimum_dissolve_delay_to_vote_seconds[0])
              ? votingPower : '-',
            dissolveDelay,
            age: neuronAge,
          };
          
          return {
            ...neurons[fixedNeuronIds],
            icpRewards: Number(icpRewards),
            ledgerRewards: Number(ledgerRewards),
            ogyRewards: Number(ogyRewards),
            dissolving: neuronState(dissolveState),
            votingPower: dissolveDelay
              > Number(neuronsParameters.neuron_minimum_dissolve_delay_to_vote_seconds[0])
              ? votingPower : '-',
            dissolveDelay,
            age: neuronAge,
          }
        });

        await Promise.all(neuronsData);
        
        setLoading(false);
        return Object.values(neurons);
      }
      setLoading(false);
    } catch (err) {
      console.log('getNeuronsByOwner error:', err);
      toast.error('Something went wrong, please retry later.', {
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
      setLoading(false);
    }
  };

  const claimAllRewards = async () => {
    setLoading(true);
    const rewardsToClaim = [];
    for (let i = 0; i < neuronsToClaim.length; i += 1) {
      if (neuronsToClaim[i].icpRewards > 0) {
        rewardsToClaim.push(claimOneReward(neuronsToClaim[i].id, 'ICP'));
      }
      if (neuronsToClaim[i].ledgerRewards > 0) {
        rewardsToClaim.push(claimOneReward(neuronsToClaim[i].id, 'GLDGov'));
      }
    }
    const isNotFulfilled = (e) => e.status !== 'fulfilled';

    Promise.allSettled(rewardsToClaim).then((res) => {
      if (res.some(isNotFulfilled)) {
        toast.error('Some of the rewards were not claimed. Please retry later.', {
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
        setLoading(false);
      }
      res.forEach((elem, i) => {
        if (elem.value.Ok) {
          setLoading(false);
          toast.success('Rewards successfully claimed', {
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
        } else {
          const hexNeuronId = uint8ArrayToHexString(neuronsToClaim[i].id);
          toast.error(`claiming neuron ${hexNeuronId} with token ${token}. ${mapResponseErrorCodeToFriendlyError(elem.value)}`, {
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
          setLoading(false);
        }
      });
      setRequestSent(true);
    });
  };

  return {
    addNeuron,
    removeNeuron,
    claimReward,
    getNeuronsByOwner,
    loading,
    requestSent,
    claimAllRewards,
    neuronError,
  };
};

export default useNeurons;
