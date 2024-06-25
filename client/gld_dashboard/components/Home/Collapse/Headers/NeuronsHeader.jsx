import Image from 'next/image';
import { useState } from 'react';

const NeuronsHeader = () => {
  const [hovered, setHovered] = useState({ disabled: false });

  const handleHover = ({ disabled, index }) => {
    setHovered({ ...hovered, disabled, index });
  };

  const headerItems = [
    { label: 'ID', iconSrc: '/svg/info.svg', tooltip: 'The unique identifier of the neuron.' },
    { label: 'State', iconSrc: '/svg/info.svg', tooltip: 'The state of the neuron.' },
    {
      label: 'Staked GLDGov',
      iconSrc: '/svg/info.svg',
      tooltip:
        'The number of GLDGov tokens staked in the neuron. The dissolve delay bonus and age bonus are applied to this value plus staked maturity in order to calculate the voting power of the neuron.',
    },
    {
      label: 'Maturity',
      iconSrc: '/svg/info.svg',
      tooltip: 'The total accumulated maturity of the neuron, regular and staked.',
    },
    {
      label: 'Dissolve Delay',
      iconSrc: '/svg/info.svg',
      tooltip:
        'The minimum time period over which the neuron owner locks up their staked GLDGov tokens. This determines how long it will take to dissolve if the neuron is placed into the Dissolving state. Once a neuron has been placed into the Dissolving state, its dissolve delay falls over the passage of time, rather like a kitchen timer, until either it is stopped or it reaches zero. When it reaches zero and enters the Dissolved state, its owner can perform a final disburse action to unlock the balance of GLDGov tokens. The dissolve delay can be configured up to a maximum of 2 years, and must be 91 days or greater for a neuron to be able to vote and earn voting rewards.',
    },
    {
      label: 'Age',
      iconSrc: '/svg/info.svg',
      tooltip:
        'The period of time that has elapsed since the neuron was created or last entered the Not Dissolving state. While dissolving, a neuron’s age is considered zero. Increasing the stake of a neuron will decrease its age. For example, if the stake is doubled, the age will be halved. Splitting a neuron creates a child neuron that inherits the age of its parent.',
    },
    { label: 'Voting Power', iconSrc: '/svg/info.svg', tooltip: 'The voting power of the neuron.' },
  ];

  return (
    <thead className="border-t-[0.5px] border-[#C6C6C6] flex flex-row items-center justify-start py-[20px] w-[100%] px-5">
      <div className={'w-10 h-10 flex justify-center items-center'}></div>
      <tr className="grid grid-cols-7 pl-3 gap-5 w-[100%]">
        {headerItems.map((item, index) => (
          <th
            key={index}
            className="w-full flex flex-row items-center justify-start min-w-40 gap-1"
          >
            <p className="text-xs font-bold text-[#D3B871]">{item.label}</p>
            <div
              className="hidden sm:flex"
              onMouseEnter={() => handleHover({ disabled: true, index })}
              onMouseLeave={() => setHovered({ disabled: false, index: null })}
            >
              <Image src={'/svg/info.svg'} className="relative" alt="info" height={20} width={20} />

              <p
                className={`${hovered.disabled && hovered.index === index ? 'font-normal text-sm tracking-tighter leading-tight z-30 absolute transform top-0 bg-black text-white px-4 py-2 rounded-lg' : 'hidden'} max-w-80`}
              >
                {item.tooltip}
              </p>
            </div>
          </th>
        ))}
      </tr>
    </thead>
  );
};

export default NeuronsHeader;
