'use client';

import { useState } from 'react';
import Image from 'next/image';
import RangeSlider from './RangeSlider';

const StakingReturn = () => {
  const [value, setValue] = useState(60);

  const handleChange = (newValue) => {
    setValue(newValue);
  };

  return (
    <article className="hidden  justify-center w-full mt-5 border-[0.5px] border-[#F3F3F3] shadow-[0_0_12px_0_#00000026] card bg-[#F3F3F3] rounded-box">
      <div className="flex flex-row pl-5 py-3 border-b-[0.5px] border-[#C6C6C6]">
        <h1 className="text-xl font-bold">Annualized Staking Return</h1>
        <div className="tooltip" data-tip="Total amount of GLDGov tokens minted.">
          <Image className="ml-2" height={18} width={18} src="/svg/info.svg" alt="info icon" />
        </div>
      </div>
      <div className="py-10">
        <RangeSlider value={value} onChange={handleChange} />
      </div>
    </article>
  );
};

export default StakingReturn;
