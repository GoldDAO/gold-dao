/* eslint-disable no-console */
/* eslint-disable camelcase */
/* eslint-disable consistent-return */
/* eslint-disable no-nested-ternary */

import {
  calculateVotingPower, hexStringToUint8Array, neuronState,
} from '../utils/functions';
import {
  convertDate,
  p,
  parseCycles,
  parseNumbers,
} from '../utils/parsers';
import { supplyData, treasuryData } from '../services/icpApi';

import useActor from './useActor';
import useCharts from './useCharts';

const useServices = () => {
  const [icpSwap] = useActor('icpSwap');
  const [ledger] = useActor('ledger');
  const [root] = useActor('root');
  const [icpNeuron] = useActor('icpNeuron');
  const [ogyNeuron] = useActor('ogyNeuron');
  const [governance] = useActor('governance');
  const [icp] = useActor('icp');
  const { setGldGovTreasury, setGldGovSupply } = useCharts();

  const gldGovPrice = async () => {
    try {
      const pools = await icpSwap.getPoolsForToken('tyyy3-4aaaa-aaaaq-aab7a-cai');

      const pool = pools?.find((po) => po?.token0Symbol === 'GOLDAO' && po?.token1Symbol === 'ICP');
      if (pool) return pool.token0Price;
      const pool2 = pools?.find(
        (po) => po?.token1Symbol === 'GOLDAO' && po?.token0Symbol === 'ICP',
      );
      return pool2?.token1Price || 0.05;
    } catch (err) {
      console.log('GOLDAO price error:', err);
      return 0.05;
    }
  };

  const gldGovTotalSupply = async () => {
    try {
      const totalSupply = await ledger.icrc1_total_supply();
      return (Number(totalSupply) / 10 ** 8).toFixed(2);
    } catch (err) {
      console.log('gldGov total supply error:', err);
      return 1000002872.7;
    }
  };

  const icpNeurons = async () => {
    try {
      const neurons = await icpNeuron.list_neurons();

      const parsed = neurons?.neurons?.active?.map((n) => {
        const diffInDays = n?.dissolving
          ? (new Date(Number(n?.dissolve_delay) * 1000).getTime()
           - new Date().getTime() - 1) / (1000 * 60 * 60 * 24)
          : Number(n?.dissolve_delay) / (60 * 60 * 24);
        return ({
          id: n?.id?.toString() || n?.id,
          dissolving: n?.dissolving,
          stakedAmount: Number(n?.staked_amount) / 10 ** 8,
          maturity: Number(n?.maturity),
          dissolveDelay: n?.dissolving
            ? `Dissolving, unlocked in ${Math.floor(diffInDays / 365.3)} years ${diffInDays % 365.3 > 0 ? `${(diffInDays % 365.3).toFixed(0) - 1} days` : ''}`
            : `Non-dissolving, locked for ${(diffInDays / 365.3).toFixed(0)} ${(diffInDays / 365.3).toFixed(0) > 1 ? 'years' : 'year'}`,
          age: 3, // FIXME harcoded
          votingPower: 8376, // FIXME harcoded
        });
      });
      return parsed;
    } catch (err) {
      console.log('icp neuron error:', err);
      return [];
    }
  };

  const ogyNeurons = async () => {
    try {
      const neurons = await ogyNeuron.list_ogy_neurons();
      // Buffer.from(uint8).toString('hex')
      const parsed = neurons?.neurons?.ogy_neurons?.map((n) => ({
        id: Buffer.from(n?.id[0].id.buffer).toString('hex'),
        dissolving: n?.dissolving,
        stakedAmount: Number(n?.cached_neuron_stake_e8s) / 10 ** 8,
        maturity: Number(n?.maturity_e8s_equivalent),
        dissolveDelay: `
          ${n?.dissolving ? 'Dissolving, unlocked in ' : 'Non-dissolving, locked for '}
          ${Number(n?.dissolve_state[0].DissolveDelaySeconds) / (365.25 * 60 * 60 * 24)}
          ${Number(n?.dissolve_state[0].DissolveDelaySeconds) / (365.25 * 60 * 60 * 24) > 1 ? ' years' : ' year'}
        `,
        age: Number(n?.aging_since_timestamp_seconds) / (365.25 * 60 * 60 * 24),
        votingPower: (Number(n?.cached_neuron_stake_e8s)
        * (1 + Number(n?.voting_power_percentage_multiplier) / 100)) / 10 ** 8,
      }));
      return parsed;
    } catch (err) {
      console.log('icp neuron error:', err);
      return [];
    }
  };

  const nervousSystemParameters = async () => {
    try {
      const getNeuronsParameters = await governance.get_nervous_system_parameters();
      return getNeuronsParameters;
    } catch (error) {
      console.log('error nervour system parameters', error);
    }
  };

  const goldNeurons = async ({ limit = 5 } = {}) => {
    try {
      const options = {
        limit,
        of_principal: [],
        start_page_at: [
          {
            id: hexStringToUint8Array(
              'fdedff3adfced06cfaddebbf224b94903423eab76cc16852d5f756d4b67bb662',
            ),
          },
        ],
      };

      const neurons = await governance.list_neurons(options);
      const neuronsParameters = await nervousSystemParameters();
      const parsed = [];
      for (let i = 0; i < neurons?.neurons?.length; i += 1) {
        const n = neurons.neurons[i];
        const neuronAge = Math.round(new Date().getTime() / 1000)
          - Number(n.aging_since_timestamp_seconds);
        const dissolveState = n.dissolve_state[0];

        const {
          votingPower, ageBonus, dissolveDelayBonus, dissolveDelay,
        } = calculateVotingPower({
          cachedNeuronStakeE8s: Number(n.cached_neuron_stake_e8s),
          stakedMaturiryE8sEquivalent: Number(
            n.staked_maturity_e8s_equivalent[0] || 0,
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

        parsed.push({
          id: n?.id[0].id,
          autoStakeMaturity: neuronsParameters?.maturity_modulation_disabled[0],
          dissolveDelayBonus,
          ageBonus,
          dateCreated: Number(n?.created_timestamp_seconds),
          dissolving: neuronState(dissolveState),
          stakedAmount: Number(n?.cached_neuron_stake_e8s || 0) / 10 ** 8,
          maturity: Number(n?.maturity_e8s_equivalent || 0),
          dissolveDelay,
          age: neuronAge,
          votingPower: dissolveDelay
            > Number(neuronsParameters.neuron_minimum_dissolve_delay_to_vote_seconds)
            ? votingPower : '-',
          totalBonus: (1 + ageBonus) * (1 + dissolveDelayBonus),
        });
      }

      return parsed;
    } catch (err) {
      console.log('gold dao neuron error:', err);
      return [];
    }
  };

  const icpTreasury = async () => {
    try {
      const treasury = await icp.account_balance_dfx({
        account: '904258703bed6394339480ab800463377bbc3f0c3d4d318cdbf041e1d523f6cf',
      });
      return (Number(treasury?.e8s) / 10 ** 8).toFixed(0);
    } catch (err) {
      console.log('icp treasury error:', err);
      return 783718; // from https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai
    }
  };

  const gldGovTreasury = async () => {
    try {
      const hexString = '7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8';

      const bytes = [];
      for (let i = 0; i < hexString.length; i += 2) {
        bytes.push(parseInt(hexString.substring(i, i + 2), 16));
      }
      const treasury = await ledger.icrc1_balance_of({
        owner: p('tr3th-kiaaa-aaaaq-aab6q-cai'),
        subaccount: [new Uint8Array(bytes)],
      });

      return parseInt((Number(treasury) / 10e7).toFixed(0), 10);
    } catch (err) {
      console.log('gldGov treasury error:', err);
      return 620000000; // from https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai
    }
  };

  const overviewData = async () => {
    try {
      const sp = await governance.get_nervous_system_parameters();
      const parsed = [
        {
          status: 'SNS Launched',
          tokenName: 'GOLDAO',
          tokenSymbol: 'GOLDAO',
          txFee: 0.001,
          initialVotingPeriod: Number(sp?.initial_voting_period_seconds?.[0]) / (60 * 60 * 24),
          maxVotingPeriod:
            (Number(sp?.wait_for_quiet_deadline_increase_seconds?.[0]) * 2) / (60 * 60 * 24),
          rejectCost: Number(sp?.reject_cost_e8s?.[0]) / 10 ** 8,
          minNeuStake: Number(sp?.neuron_minimum_stake_e8s?.[0]) / 10 ** 8,
          minDissDelay: (
            Number(sp?.neuron_minimum_dissolve_delay_to_vote_seconds?.[0])
            / (60 * 60 * 24)
          ).toFixed(0),
          maxDissDelay: Number(sp?.max_dissolve_delay_seconds?.[0]) / (365.25 * 60 * 60 * 24),
          maxDissDelayBonus: Number(sp?.max_dissolve_delay_bonus_percentage?.[0]),
          maxAxABonus: Number(sp?.max_neuron_age_for_age_bonus?.[0]) / (365.25 * 60 * 60 * 24),
          maxAgeBonus: Number(sp?.max_age_bonus_percentage?.[0]),
          rewardRate:
            Number(sp?.voting_rewards_parameters?.[0]?.initial_reward_rate_basis_points?.[0]) / 100,
          icpTreasury: await icpTreasury(),
          gldGovTreasury: await gldGovTreasury(),
        },
      ];
      return parsed;
    } catch (err) {
      console.log('overview data error:', err);
      return [];
    }
  };

  const getCanisters = async () => {
    try {
      const canisters = await root.get_sns_canisters_summary({ update_canister_list: [] });
      const keys = Object.keys(canisters);
      const parsed = keys.map((k) => {
        if (canisters[k].length === 0) {
          return null;
        }
        return {
          type: k.replace(/^\w/, (c) => c.toUpperCase()),
          id: canisters[k][0]?.canister_id?.toString(),
          cycles: parseCycles(Number(canisters[k][0]?.status?.[0]?.cycles ?? 0)).toFixed(4),
          memory: (Number(canisters[k][0]?.status?.[0]?.memory_size ?? 0) / 1048576).toFixed(2),
          status: Object.keys(canisters[k][0]?.status?.[0]?.status ?? { 'out of cycles': null })?.[0]?.replace(/^\w/, (c) => c.toUpperCase()),
          idleCycles:
            parseNumbers(Number(canisters[k][0]?.status?.[0]?.idle_cycles_burned_per_day ?? 0)),
          freezingCycles: (
            (Number(canisters[k][0]?.status?.[0]?.idle_cycles_burned_per_day ?? 0) / (24 * 3600))
            * (Number(canisters[k][0]?.status?.[0]?.settings.freezing_threshold ?? 0) / 10e11)
          ).toFixed(4),
          ModuleHash: canisters[k][0].status[0]?.module_hash[0],
          freezing_threshold:
            Number(canisters[k][0]?.status?.[0]?.settings?.freezing_threshold ?? 0),
          controllers: canisters[k][0].status[0]?.settings.controllers[0].toString(),
        };
      });
      return parsed.filter((canister) => canister !== null);
    } catch (err) {
      console.log('get canisters error:', err);
    }
  };

  // collapses

  const getProposals = async ({ limit = 5, before_proposal = [] } = {}) => {
    try {
      const options = {
        limit,
        include_reward_status: [],
        exclude_type: [],
        include_status: [],
        before_proposal, // no incluye el seleccionado. trae los anteriores a este
      };
      const props = await governance.list_proposals(options);
      const data = props?.proposals?.map((prop) => ({
        id: Number(prop.id?.[0]?.id),
        title: prop?.proposal[0]?.title,
        votes: {
          yes: Number(prop?.latest_tally?.[0]?.yes),
          no: Number(prop?.latest_tally?.[0]?.no),
          total: Number(prop?.latest_tally?.[0]?.total),
        },
        status: Number(prop?.executed_timestamp_seconds)
          ? 'Executed'
          : Number(prop?.failed_timestamp_seconds)
            ? 'Failed'
            : 'Open',
        url: `https://nns.ic0.app/proposal/?u=tw2vt-hqaaa-aaaaq-aab6a-cai&proposal=${Number(prop.id?.[0]?.id)}`,
        topic: Object.keys(prop?.proposal[0]?.action?.[0])?.[0]
          ?.split(/(?=[A-Z])/)
          .join(' ')
          .replace('Sns', 'SNS'),
      }));

      return data;
    } catch (err) {
      console.log('get proposal error:', err);
      return [];
    }
  };

  const getTxs = async ({ start = 0, length = 5 } = {}) => {
    try {
      const res = await ledger.get_transactions({
        start: 0,
        length: 0,
      });

      const logLength = (Number(res?.log_length) || length) - length;
      const transactions = await ledger.get_transactions({
        start: start || logLength,
        length,
      });

      const data = transactions?.transactions
        ?.map((t, i) => ({
          index: start && length ? Number(transactions.first_index) + i : logLength + i,
          amount: (Number(t[t.kind]?.[0]?.amount) || 0) / 10 ** 8,
          type: t?.kind,
          from: t[t.kind]?.[0]?.from?.owner?.toString(),
          to: t[t.kind]?.[0]?.to?.owner?.toString(),
          timestamp: convertDate(Number(t.timestamp)),
        }))
        ?.reverse?.();

      return data;
    } catch (err) {
      console.log('get txs error:', err);
      return [];
    }
  };

  // charts
  const getTreasuryChart = async () => {
    try {
      const data = await treasuryData();
      setGldGovTreasury(data); // zustand setter
    } catch (err) {
      console.log('getTreasuryChart error', err);
    }
  };

  const getSupplyChart = async () => {
    try {
      const data = await supplyData();
      setGldGovSupply(data); // zustand setter
    } catch (err) {
      console.log('getSupplyChart error', err);
    }
  };

  return {
    gldGovPrice,
    gldGovTotalSupply,
    icpNeurons,
    ogyNeurons,
    goldNeurons,
    overviewData,
    gldGovTreasury,
    icpTreasury,
    getProposals,
    getTxs,
    getCanisters,
    getTreasuryChart,
    getSupplyChart,
  };
};

export default useServices;
