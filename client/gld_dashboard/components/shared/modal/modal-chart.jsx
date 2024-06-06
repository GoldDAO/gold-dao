'use client';

import { useState } from 'react';
import { calculateTimestamp, verifyTimestamp } from '../../../utils/functions';

import Graph from '../../Home/Graph';
import useCharts from '../../../hooks/useCharts';

export default function ModalChart({ name }) {
  const [selectedTimestamp, setSelectedTimestamp] = useState({ date: '1M', t: 86400 * 31 });
  const { setSelectedDistance } = useCharts();

  const dates = [
    // { name: "1J", timestamp: calculateTimestamp(86400 * 2) },
    { name: '1 WEEK', timestamp: calculateTimestamp(86400 * 7) },
    { name: '1 MONTH', timestamp: calculateTimestamp(86400 * 31) },
    { name: '3 MONTHS', timestamp: calculateTimestamp(86400 * 31 * 3) },
    // { name: "6M", timestamp: calculateTimestamp(86400 * 31 * 6) },
    // { name: "AAJ", timestamp: calculateTimestamp() },
    { name: '1 YEAR', timestamp: calculateTimestamp(86400 * 365) },
    // { name: "2A", timestamp: calculateTimestamp(86400 * 365 * 2) },
    // { name: "5A", timestamp: calculateTimestamp(86400 * 365 * 5) },
    // { name: "10A", timestamp: calculateTimestamp(86400 * 365 * 10) },
    { name: 'ALL', timestamp: calculateTimestamp('ALL') },
  ];

  return (
    <div>
      <div className="h-20"></div>

      <div className="w-full h-fit flex border-b-[0.5px] border-t-[0.5px]">
        {dates.map(({ timestamp }) => (
          <button
            className={`
          text-center text-xs w-full ${selectedTimestamp.date === name ? 'bg-DarkGrey text-white font-bold' : ''} py-2`}
            key={name}
            onClick={() => {
              setSelectedTimestamp({ date: name, t: timestamp });
              setSelectedDistance(timestamp);
            }}
          >
            {name}
          </button>
        ))}
      </div>
      <div className="h-20"></div>

      <Graph name={name} timestamp={verifyTimestamp(selectedTimestamp.t)} />
    </div>
  );
}
