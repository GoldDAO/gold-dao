import Image from 'next/image';
import Graph from './Graph';
import Modal from '../shared/modal/modal';
import ModalChart from '../shared/modal/modal-chart';
import { calculateTimestamp } from '../../utils/functions';
import useCharts from '../../hooks/useCharts';

const Chart = ({
  name,
  amount,
}) => {
  const { setSelectedDistance, selectedDistance } = useCharts();

  const dates = [
    { name: '1 WEEK', timestamp: calculateTimestamp(86400 * 7) },
    { name: '1 MONTH', timestamp: calculateTimestamp(86400 * 31) },
    { name: '3 MONTHS', timestamp: calculateTimestamp(86400 * 31 * 3) },
    { name: '1 YEAR', timestamp: calculateTimestamp(86400 * 365) },
    { name: 'ALL', timestamp: calculateTimestamp('ALL') },
  ];

  const toolTips = {
    Treasury: 'GLDGov tokens which are at the disposition of the Gold DAO SNS DAO, allocated by the Gold DAO decentralization swap.',
    Staked: 'GLDGov tokens which are staked in the SNS as Gold DAO neurons.',
    Liquid: 'GLDGov tokens that are publicly available and not locked. I.e. tokens which are not staked, held by the team or allocated for reward distribution.',
    Burned: 'GLDGov tokens that have been burned and are permanently taken out of existence.',
    Holders: 'The number of unique accounts that hold GLDGov tokens.',
  };

  return (
    <section className={' w-[67%]  p-5 relative hidden sm:block'}>
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
                  toolTips[name]
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
            {amount}
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
              text-center text-xs w-full ${selectedDistance.name === date.name ? 'bg-DarkGrey text-white font-bold' : ''} py-2`}
            key={date.name}
            onClick={() => {
              setSelectedDistance({ name: date.name, timestamp: date.timestamp });
            }}
          >
            {date.name}
          </button>
        ))}
      </div>
      <Graph name={name} timestamp={selectedDistance} />
      <Modal title={`chart ${name}`} idModal="chartmodalG">
        <ModalChart name={name} />
      </Modal>
    </section>
  );
};

export default Chart;
