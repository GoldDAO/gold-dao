'use client';

import React, { useEffect, useState } from 'react';
import { Spinner, Tooltip } from '@nextui-org/react';
import useServices from '../../hooks/useServices';
import { InfoIcon } from '../../utils/svgs.jsx';
import { parseNumbers } from '../../utils/parsers';

const OverviewTable = () => {
  const { overviewData } = useServices();
  const [overviewItems, setOverviewItems] = useState({ loading: true, data: [] });

  useEffect(() => {
    (async () => {
      const data = await overviewData();
      setOverviewItems({ loading: false, data });
    })();
  }, []);
  const tooltipClassName = {
    base: [
      'max-w-[400px]',
    ],
    content: [
      'py-2 px-4 shadow-xl',
      'text-white bg-black',
    ],
  };
  return (
    <div>
      <div className='text-start sm:mx-5 font-bold text-xl border-[0.5px] border-b-0 rounded-t-4xl h-19 border-DarkGrey'>
        <h2 className='mx-4 mt-6'>Overview</h2>
      </div>
      {overviewItems.loading ? (
        <div className='flex items-center justify-center gap-x-4 sm:text-xl'>
          <Spinner className='' classNames={{ circle1: 'border-b-Beige', circle2: 'border-b-Beige' }} />
          <h4>
            Loading overview
          </h4>
        </div>)
        : (<div className="text-start grid grid-cols-2 sm:grid-cols-4 sm:mx-5 mb-5 text-xs sm:text-base font-normal capitalize">
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Status</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='SNS Launched The decentralization swap for the SNS has completed successfully and the SNS has launched'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].status}</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Token Name</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The name of the token.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].tokenName}</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Token Symbol</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The symbol of the token.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].tokenSymbol}</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Transaction Fee</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The default GLDGov transaction fee.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {overviewItems.data[0].txFee}
              <span className='font-normal ml-2'>{overviewItems.data[0].tokenSymbol}</span>
            </h2>
          </div>

          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Initial Voting Period</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content="The initial voting period of a proposal. A proposal's voting period may be extended during a proposal's lifecycle due to the wait-for-quiet algorithm."
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].initialVotingPeriod} days</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>max voting period extension</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The maximum total voting period extension. The wait-for-quiet algorithm extends the voting period of a proposal when there is a flip in the majority vote. The total duration of all such extensions cannot exceed this maximum.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].maxVotingPeriod} days</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>reject cost</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The number of GLDGov tokens charged to the proposer if a proposal is rejected.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {overviewItems.data[0].rejectCost}
              <span className='font-normal ml-2'>{overviewItems.data[0].tokenSymbol}</span>
            </h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Min neuron stake</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The minimum number of GLDGov tokens that can be staked in a neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {overviewItems.data[0].minNeuStake}
              <span className='font-normal ml-2'>{overviewItems.data[0].tokenSymbol}</span>
            </h2>
          </div>

          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>min dissolve delay to vote</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The minimum dissolve delay that a neuron must have to be eligible to vote on proposals. Dissolve delay is the minimum time period over which the neuron owner locks up their staked GLDGov tokens.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].minDissDelay} days</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>max dissolve delay</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The maximum dissolve delay that a neuron can have. Dissolve delay is the minimum time period over which the neuron owner locks up their staked GLDGov tokens.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].maxDissDelay} years</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>max dissolve delay bonus</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The maximum dissolve delay bonus percentage, i.e., the dissolve delay bonus of a neuron that has a dissolve delay of “Max Dissolve Delay”. Dissolve delay bonus is a boost to voting power and is a linear function of the dissolve delay.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].maxDissDelayBonus} %</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-b-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>max age for age bonus</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The age at which the maximum age bonus will be given. Neurons with an age greater than this will be treated as if they are this age. The age of a neuron is the period of time that has elapsed since a neuron was created or last entered the Not Dissolving state.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {overviewItems.data[0].maxAxABonus} years
            </h2>
          </div>

          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-r-0 border-DarkGrey sm:rounded-bl-4xl'>
            <div className='flex items-center gap-x-2'>
              <h4>Max Age Bonus</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='The maximum age bonus percentage, i.e., the age bonus of a neuron that has an age of “Max Age for Age Bonus” or older. Age bonus is a boost to voting power and is a linear function of the age of a neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].maxAgeBonus} %</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-r-0 border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>Reward Rate</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content={
                    <div className="flex flex-col items-start gap-1">
                      <p>The voting reward rate.</p>
                      <p>
                        If voting rewards are enabled for an SNS,
                        three parameters determine the voting reward
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
                          The voting reward rate transition period,
                          the amount of time over which the voting
                          reward rate changes (e.g., decreases) from the
                          initial rate to the final rate.
                        </p>
                      </div>
                      <p>
                        The allocation of voting rewards is
                        determined by multiplying the voting reward rate by
                        the total supply of tokens
                        (on an annualized basis). For the first voting reward event,
                        the reward rate is the initial
                        voting reward rate, and the rate changes quadratically
                        over the voting reward rate
                        transition period, gradually leveling out, until the end of
                        the period when it reaches the
                        final voting reward rate, where it remains from that
                        point forward.
                      </p>
                    </div>
                  }
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>{overviewItems.data[0].rewardRate} %</h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-r-0 border-t-0 sm:border-t-[0.5px] border-DarkGrey'>
            <div className='flex items-center gap-x-2'>
              <h4>ICP Treasury</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='ICP tokens which are at the disposition of the Gold DAO SNS DAO, raised by the Gold DAO decentralization swap.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {parseNumbers(overviewItems.data[0].icpTreasury)}
              <span className='font-normal ml-2'>ICP</span>
            </h2>
          </div>
          <div className='flex flex-col gap-y-4 py-6 px-5 border-[0.5px] border-t-0 sm:border-t-[0.5px] border-DarkGrey sm:rounded-br-4xl'>
            <div className='flex items-center gap-x-2'>
              <h4>GLDGov Treasury</h4>
              <div className='hidden sm:block'>
                <Tooltip
                  content='GLDGov tokens which are at the disposition of the Gold DAO SNS DAO, allocated by the Gold DAO decentralization swap.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </div>
            <h2 className='text-sm sm:text-xl font-bold'>
              {parseNumbers(overviewItems.data[0].gldGovTreasury)}
              <span className='font-normal ml-2'>GLDGov</span>
            </h2>
          </div>
        </div>)}
    </div >
  );
};

export default OverviewTable;
