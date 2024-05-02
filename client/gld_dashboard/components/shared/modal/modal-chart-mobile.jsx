'use client';

import Image from 'next/image';
import { useState } from 'react';
import { calculateTimestamp, verifyTimestamp } from '../../../utils/functions';

import Graph from '../../Home/Graph.jsx';
import { useCharts } from '../../../hooks/useCharts';

export default function ModalChartMobile({ name }) {
  const [selectedTimestamp, setSelectedTimestamp] = useState({ date: '1M', t: 86400 * 31 });
  const { setSelectedDistance } = useCharts();

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
        {name !== 'Treasury' && name !== 'Total GLDGov Supply' && (
          <div className="absolute top-0 w-full h-full flex justify-center items-center bg-black bg-opacity-20 z-50 rounded-t-[24px]">
            <div className="text-white text-4xl font-bold flex justify-center items-center gap-2">
              <p>Coming Soon</p>
              <div
                className="tooltip "
                data-tip={"Good news! We're working on this. Coming your way soon."}
              >
                <Image src={'svg/infoWhite.svg'} alt="f" height={30} width={30} />
              </div>
              <form method="dialog" className="absolute top-10 right-7">
                <button className="bg-transparent h-[24px] w-[26px] rounded-full flex justify-center items-center p-2 outline-none">
                </button>
              </form>
            </div>
          </div>
        )}

        <div className="w-full h-fit  flex  border-b-[0.5px]  border-t-[0.5px]">
          {dates.map((date) => (
            <button
              className={`
              text-center text-xs w-full ${selectedTimestamp.date === date.name ? 'bg-DarkGrey text-white font-bold' : ''} py-2`}
              key={date.name}
              onClick={() => {
                setSelectedTimestamp({ date: date.name, t: date.timestamp });
                setSelectedDistance(date.timestamp);
              }}
            >
              {date.name}
            </button>
          ))}
        </div>
      </div>
      <section className="mb-6">
        <Graph name={name} timestamp={verifyTimestamp(selectedTimestamp.t)} />
      </section>
      <span className="text-base text-[#C6C6C6]">Total amount of GLDGov tokens minted.</span>
    </div>
  );
}
