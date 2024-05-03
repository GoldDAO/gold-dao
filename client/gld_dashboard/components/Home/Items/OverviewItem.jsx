import Image from 'next/image';
import { useState } from 'react';
import { parseNumbers } from '../../../utils/parsers';
import NavbarModal from '../../shared/modal/NavbarModal.jsx';

const OverviewItem = ({
  status,
  tokenName,
  tokenSymbol,
  txFee,
  initialVotingPeriod,
  maxVotingPeriod,
  rejectCost,
  minNeuStake,
  minDissDelay,
  maxDissDelay,
  maxDissDelayBonus,
  maxAxABonus,
  maxAgeBonus,
  rewardRate,
  icpTreasury,
  gldGovTreasury,
}) => {
  const [hovered, setHovered] = useState({ dissabled: false, index: null });

  const handleHover = ({ dissabled, index }) => {
    setHovered({ ...hovered, dissabled, index });
  };

  const handleLeave = () => {
    setHovered({ dissabled: false, index: null });
  };

  const overviewData = [
    {
      label: 'Status',
      value: status,
      tooltip:
        'SNS Launched The decentralization swap for the SNS has completed successfully and the SNS has launched.',
      style: 'top-10 left-60',
    },
    {
      label: 'Token Name',
      value: tokenName,
      tooltip: 'The name of the token.',
      style: 'top-10 left-10',
    },

    {
      label: 'Token Symbol',
      value: tokenSymbol,
      tooltip: 'The symbol of the token.',
      style: 'top-10 left-10',
    },
    {
      label: 'Transaction Fee',
      value: `${txFee} GLDGov`,
      tooltip: 'The default GLDGov transaction fee.',
      style: 'top-10 left-10',
    },
    {
      label: 'Initial Voting Period',
      value: `${initialVotingPeriod} days`,
      style: '-top-20 left-80',
      tooltip:
        'The initial voting period of a proposal. A proposal’s voting period may be extended during a proposal’s lifecycle due to the wait-for-quiet algorithm.',
    },
    {
      label: 'Max Voting Period Extension',
      value: `${maxVotingPeriod} days`,
      tooltip:
        'The maximum total voting period extension. The wait-for-quiet algorithm extends the voting period of a proposal when there is a flip in the majority vote. The total duration of all such extensions cannot exceed this maximum.',
    },

    {
      label: 'Reject Cost',
      value: `${rejectCost} GLDGov`,
      tooltip: 'The number of GLDGov tokens charged to the proposer if a proposal is rejected.',
    },
    {
      label: 'Min Neuron Stake',
      value: `${minNeuStake} GLDGov`,
      tooltip: 'The minimum number of GLDGov tokens that can be staked in a neuron.',
    },
    {
      label: 'Min Dissolve Delay to Vote',
      value: `${minDissDelay} days`,
      style: '-top-20 left-80',
      tooltip: (
        <div>
          <p>
            The minimum dissolve delay that a neuron must have to be eligible to vote on proposals.
          </p>
          <br />
          <p>
            Dissolve delay is the minimum time period over which the neuron owner locks up their
            staked GLDGov tokens.
          </p>
        </div>
      ),
    },
    {
      label: 'Max Dissolve Delay',
      value: `${maxDissDelay} years`,
      tooltip: (
        <div>
          <p>The maximum dissolve delay that a neuron can have.</p>
          <br />
          <p>
            Dissolve delay is the minimum time period over which the neuron owner locks up their
            staked GLDGov tokens.
          </p>
        </div>
      ),
    },
    {
      label: 'Max Dissolve Delay Bonus',
      value: `${maxDissDelayBonus}%`,
      tooltip: (
        <div>
          <p>
            The maximum dissolve delay bonus percentage, i.e., the dissolve delay bonus of a neuron
            that has a dissolve delay of “Max Dissolve Delay”.
          </p>
          <br />
          <p>
            Dissolve delay bonus is a boost to voting power and is a linear function of the dissolve
            delay.
          </p>
        </div>
      ),
    },
    {
      label: 'Max Age for Age Bonus',
      value: `${maxAxABonus} years`,
      tooltip: (
        <div>
          <p>
            The age at which the maximum age bonus will be given. Neurons with an age greater than
            this will be treated as if they are this age.
          </p>
          <br />
          <p>
            The age of a neuron is the period of time that has elapsed since a neuron was created or
            last entered the Not Dissolving state.
          </p>
        </div>
      ),
      style: '-top-20 left-40',
    },
    {
      label: 'Max Age Bonus',
      value: `${maxAgeBonus}%`,
      style: '-top-20 left-80',
      tooltip: (
        <div>
          <p>
            The maximum age bonus percentage, i.e., the age bonus of a neuron that has an age of
            “Max Age for Age Bonus” or older.
          </p>
          <br />
          <p>
            Age bonus is a boost to voting power and is a linear function of the age of a neuron.
          </p>
        </div>
      ),
    },
    {
      label: 'Reward Rate',
      value: `${rewardRate}%`,
      style: '-top-80 left-0',
      tooltip: (
        <div className="flex flex-col items-start gap-1">
          <p>The voting reward rate.</p>
          <p>
            If voting rewards are enabled for an SNS, three parameters determine the voting reward
            rate:
          </p>
          <div className="w-full ml-2">
            <p className="flex justify-start">
              {' '}
              <span className="">1.</span>The initial voting reward rate.
            </p>
            <p className="flex justify-start">
              <span className="">2.</span>The final voting reward rate.
            </p>
            <p className="flex justify-start">
              <span className="">3.</span>
              The voting reward rate transition period, the amount of time over which the voting
              reward rate changes (e.g., decreases) from the initial rate to the final rate.
            </p>
          </div>
          <p>
            The allocation of voting rewards is determined by multiplying the voting reward rate by
            the total supply of tokens (on an annualized basis). For the first voting reward event,
            the reward rate is the initial voting reward rate, and the rate changes quadratically
            over the voting reward rate transition period, gradually leveling out, until the end of
            the period when it reaches the final voting reward rate, where it remains from that
            point forward.
          </p>
        </div>
      ),
    },
    {
      label: 'ICP Treasury',
      value: `${parseNumbers(icpTreasury)} ICP`,
      style: '-top-20 -left-0',
      tooltip:
        'ICP tokens which are at the disposition of the Gold DAO SNS DAO, raised by the Gold DAO decentralization swap.',
    },
    {
      label: 'GLDGov Treasury',
      value: `${parseNumbers(gldGovTreasury)} GLDGov`,
      style: '-top-20 -left-0',
      tooltip:
        'GLDGov tokens which are at the disposition of the Gold DAO SNS DAO, allocated by the Gold DAO decentralization swap.',
    },
  ];
  return (
    <div>
      <div className="text-start divide-y grid grid-cols-2 sm:grid-cols-4 sm:w-full">
        {overviewData.map((item, index) => (
          <div
            key={item.label}
            className={`border border-gray-[#C6C6C6] py-6 px-1 sm:px-5 relative ${index % 4 === 3 ? 'border-r-0' : 'border-r'} ${index % 4 === 0 || index === 5 || index === 9 ? 'border-l-0' : 'border-l'}`}
          >
            <div className="flex flex-row pb-3 ">
              <h4 className="ml-3 whitespace-nowrap text-[10px] sm:text-[16px] truncate">{item.label}</h4>
              <div
                className="hidden sm:flex"
                onMouseEnter={() => handleHover({ dissabled: true, index })}
                onMouseLeave={handleLeave}
              >
                <Image
                  alt="More info"
                  height={18}
                  width={18}
                  className="ml-3 cursor-pointer"
                  src="svg/info.svg"
                />
                <div
                  className={`${hovered.dissabled === true && hovered.index === index ? `absolute  transform -translate-x-1/2 bg-black text-white px-4 py-5 rounded z-20 w-[350px] ${item.style}` : 'hidden'}   
             `}
                >
                  <h1 className="font-bold text-sm">{item.label}</h1>
                  <br />
                  <div className="text-xs"> {item.tooltip}</div>
                </div>
              </div>
            </div>
            <h1 className="text-sm sm:text-xl ml-3 whitespace-nowrap">
              {item.value.endsWith('GLDGov') || item.value.endsWith('ICP') ? (
                <div className="sm:text-[20px]">
                  {item.value.length === 6 ? (
                    <span className="font-bold">{item.value}</span>
                  ) : (
                    <>
                      <span className="font-bold">
                        {item.value.replace('GLDGov', '').replace('ICP', '')}
                      </span>
                      {item.value.endsWith('GLDGov') ? 'GLDGov' : 'ICP'}
                    </>
                  )}
                </div>
              ) : (
                <span className="font-bold ">{item.value}</span>
              )}
            </h1>
          </div>
        ))}
      </div>
      <NavbarModal />
    </div>
  );
};

export default OverviewItem;
