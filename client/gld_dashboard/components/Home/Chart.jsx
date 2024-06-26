import Image from 'next/image';
import { useState } from 'react';
import Graph from './Graph';
import Modal from '../shared/modal/modal';
import ModalChart from '../shared/modal/modal-chart';
import { calculateTimestamp } from '../../utils/functions';
import useCharts from '../../hooks/useCharts';

const Chart = ({ name, amount }) => {
  const { setSelectedDistance } = useCharts();
  const [selectedTimestamp, setSelectedTimestamp] = useState({ date: '3M', t: 86400 * 31 * 3 });

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
    <section className={' w-[67%]  p-5 relative hidden sm:block'}>
      {name !== 'Treasury' && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center bg-black bg-opacity-20  z-50  rounded-r-[36px]">
          <div className="text-white text-4xl font-bold flex justify-center items-center gap-2">
            <p>Coming Soon</p>
            <div
              className="tooltip "
              data-tip={"Good news! We're working on this. Coming your way soon."}
            >
              <Image src={'svg/infoWhite.svg'} alt="f" height={30} width={30} />
            </div>
          </div>
        </div>
      )}
      <div>
        <div className="flex justify-between">
          <div className="w-full flex justify-between border-b border-[#C6C6C6] pb-5">
            <div className="flex justify-center items-center gap-2">
              <div>
                <h1 className="text-4xl font-bold">{name}</h1>
              </div>
              <div
                className="tooltip "
                data-tip={
                  'GLDGov tokens which are at the disposition of the Gold DAO SNS DAO, allocated by the Gold DAO decentralization swap.'
                }
              >
                <Image src="svg/info.svg" alt="" height={25} width={25} />
              </div>
            </div>
            {/* <div className="flex justify-center items-center
             rounded-full bg-[#C6C6C6] h-8 w-8 ">
              <Image
                alt="expand data"
                height={10}
                width={10}
                src="svg/expand.svg"
              />
            </div> */}
          </div>
        </div>
        <div className="pt-5">
          <h1 className="flex flex-row mb-5 text-4xl font-bold">
            {name !== 'Treasury' ? 'N/A' : amount || '0.0'}
            {name !== 'Treasury' ? null : (
              <Image width={25} height={25} src="svg/g-logo.svg" className="ml-2" alt="gold dao" />
            )}
          </h1>
        </div>
      </div>
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
      <Graph name={name} timestamp={selectedTimestamp.t} />
      <Modal title={`chart ${name}`} idModal="chartmodalG">
        <ModalChart name={name} />
      </Modal>
    </section>
  );
};

export default Chart;
