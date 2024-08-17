'use client';

import Image from 'next/image';
import { useState } from 'react';
import useBalances from '../../../hooks/useBalances';
import { parseNumbers } from '../../../utils/parsers';

export default function RewardsCards({
  title,
  value,
  setModalTitle,
  setAmount,
  svg,
  status,
  setIcp,
  setGold,
  setOgy,
}) {
  let rewardValue = value;
  if (rewardValue !== 0) {
    rewardValue /= 10 ** 8;
  }
  const openModal = (clickedTitle) => {
    setAmount(value);
    setModalTitle(clickedTitle);
    document.getElementById('my_modal_1').showModal();
  };
  const [loading, setLoading] = useState(false);
  const { getBalance } = useBalances();

  // const additionalStatus = rewardValue <= 0 ? false : status;

  const handleReloadClick = async () => {
    setLoading(true);

    var amount;
    if (title === 'ICP') {
      amount = await getBalance('icp');
      setIcp({loading: false, amount});
    } else if (title === 'OGY') {
      amount = await getBalance('ogy');
      setOgy({loading: false, amount});
    } else {
      amount = await getBalance('ledger');
      setGold({loading: false, amount});
    }

    setAmount(amount);
    setLoading(false);
  };

  return (
    <div
      className={`cardshadow flex flex-col justify-center border-[#c6c6c6] border w-full sm:w-full h-[8rem] sm:h-[200px] rounded-[2rem] bg-[${status ? '#F3F3F3' : '#F3F3F3'}] ${status ? '' : 'pointer-events-none opacity-50'} ${status ? '' : 'blur-[12]'}`}
    >
      <div className="flex p-6 items-center justify-between w-full relative h-[30%]">
        <h1 className="text-[#000000] font-medium text-xl">{title} Balance</h1>

        <div
          className={`bg-[#C6C6C6] hidden sm:flex rounded-full h-10 w-10  justify-center items-center cursor-pointer ${loading ? 'animate-spin' : ''}`}
          onClick={handleReloadClick}
        >
          <Image src={'/svg/reload.svg'} alt="" height={15} width={15} />
        </div>
      </div>

      <div className="flex p-6 items-center justify-between w-full relative h-[30%] sm:h-[50%]">
        <div className="text-[2rem]  sm:text-[3rem] font-bold flex gap-4 justify-center items-center">
          {loading ? '...' : parseNumbers(rewardValue.toFixed(2))}
          <Image
            width={8}
            height={8}
            src={svg}
            className="w-6 h-6 sm:w-8 sm:h-8"
            alt="iconbalance"
          />
        </div>

        <Image
          src="svg/arrows.svg"
          height={50}
          width={50}
          onClick={() => openModal(title)}
          className="cursor-pointer w-8 sm:w-10 h-8 sm:h-10"
          alt="arrows icon"
        />
      </div>
    </div>
  );
}
