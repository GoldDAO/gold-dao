'use client';

import { useState } from 'react';
import { calculateTimestamp, verifyTimestamp } from '../../../utils/functions';

import Graph from '../../Home/Graph';
import useCharts from '../../../hooks/useCharts';

export default function ModalChartMobile({ name }) {
  const { setSelectedDistance, selectedDistance } = useCharts();

  const dates = [
    // { name: "1J", timestamp: calculateTimestamp(86400 * 2) },
    { name: '1 W', timestamp: calculateTimestamp(86400 * 7) },
    { name: '1 M', timestamp: calculateTimestamp(86400 * 31) },
    { name: '3 M', timestamp: calculateTimestamp(86400 * 31 * 3) },
    // { name: "6M", timestamp: calculateTimestamp(86400 * 31 * 6) },
    // { name: "AAJ", timestamp: calculateTimestamp() },
    { name: '1 Y', timestamp: calculateTimestamp(86400 * 365) },
    // { name: "2A", timestamp: calculateTimestamp(86400 * 365 * 2) },
    // { name: "5A", timestamp: calculateTimestamp(86400 * 365 * 5) },
    // { name: "10A", timestamp: calculateTimestamp(86400 * 365 * 10) },
    { name: 'ALL', timestamp: calculateTimestamp('ALL') },
  ];

  return (
    <div className="">
      <div className="w-full h-fit flex justify-evenly rounded-[36px] border-[0.5px] mt-20">

        <div className="w-full h-fit  flex  border-b-[0.5px]  border-t-[0.5px]">
          {dates.map((date) => (
            <button
              className={`
              text-center text-xs w-full ${selectedDistance.name === date.name ? 'bg-DarkGrey text-white font-bold' : ''} py-2`}
              key={date.name}
              onClick={() => {
                setSelectedDistance({name: date.name, timestamp: date.timestamp});
              }}
            >
              {date.name}
            </button>
          ))}
        </div>
      </div>
      <section className="mb-6">
        <Graph key="mobile-graph" name={name} timestamp={selectedDistance} />
      </section>
      <span className="text-base text-[#C6C6C6]">Total amount of GLDGov tokens minted.</span>
    </div>
  );
}
