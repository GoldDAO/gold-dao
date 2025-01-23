import Image from 'next/image';
import { useState } from 'react';

const TransactionsHeader = () => {
  // Definir un array de objetos que contenga la información de cada columna
  const [hovered, setHovered] = useState({ disabled: false });

  const handleHover = ({ disabled, index }) => {
    setHovered({ ...hovered, disabled, index });
  };

  const headerItems = [
    {
      label: 'Index',
      infoSrc: '/svg/info.svg',
      tooltip: 'The index of the transaction in the GOLDAO ledger.',
    },
    {
      label: 'Amount',
      infoSrc: '/svg/info.svg',
      tooltip:
        "The amount of GOLDAO tokens transferred, or for 'approve' transactions, the designated amount of GOLDAO tokens that the 'Spender Account' is authorized to transfer on behalf of the 'From' account.",
    },
    {
      label: 'Type',
      infoSrc: '/svg/info.svg',
      tooltip: 'The type of GOLDAO transaction (i.e., mint, burn, approve, or transfer).',
    },
    {
      label: 'Timestamp',
      infoSrc: '/svg/info.svg',
      tooltip: 'The date the GOLDAO ledger constructed the block containing the transaction.',
    },
    {
      label: 'From',
      infoSrc: '/svg/info.svg',
      tooltip:
        "The account that GOLDAO tokens were transferred from, or for 'approve' transactions, the account whose owner has authorized the 'spender' account to transfer a designated amount of GOLDAO tokens from the account on their behalf.",
    },
    {
      label: 'To',
      infoSrc: '/svg/info.svg',
      tooltip:
        "The account that GOLDAO tokens were transferred to. Not applicable for 'approve' transactions.",
    },
  ];

  return (
    <thead className="border-t-[0.5px] border-[#C6C6C6] flex flex-row items-center justify-around py-[20px] w-[1200px] sm:w-[100%] px-10 ">
      <tr className="grid grid-cols-6 w-[1200] sm:w-[100%] ">
        {headerItems.map((item, index) => (
          <th
            key={index}
            className="w-full flex flex-row items-center  justify-start min-w-40 gap-1"
          >
            <p className="text-xs sm:text-xs font-bold text-[#D3B871] ">{item.label}</p>
            <div
              className="hidden sm:flex"
              onMouseEnter={() => handleHover({ disabled: true, index })}
              onMouseLeave={() => setHovered({ disabled: false, index: null })}
            >
              <Image src={'/svg/info.svg'} className="relative" alt="info" height={20} width={20} />
              <p
                className={`${hovered.disabled && hovered.index === index ? 'text-sm tracking-tighter leading-tight z-30 absolute font-normal transform top-0 bg-black text-white px-4 py-2 rounded-lg' : 'hidden'} max-w-80`}
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

export default TransactionsHeader;
