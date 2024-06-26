'use client';

import React, { useEffect, useState } from 'react';

import Image from 'next/image';

const RangeSlider = ({ step, onChange }) => {
  const [max] = useState(100);
  const [min] = useState(10);
  const [value, setValue] = useState(30);
  const [percentage, setPercentage] = useState((value / 1000) * 100);

  const handleChange = (event) => {
    const newValue = parseInt(event.target.value, 10);

    const clampedValue = Math.max(newValue, 20);
    setValue(clampedValue);
    onChange(clampedValue);
  };

  useEffect(() => {
    setPercentage((value / 1000) * 100);
  }, [value]);

  return (
    <div className="flex justify-between w-[100%] pr-10 pl-10">
      <div className="relative w-[85%]">
        <div className="border-2 absolute top-0 border-[#D3B871] z-0 text-xl  h-[50px] w-[100%] font-medium text-blue-100 text-center pl-10 pr-0.5 cursor-pointer leading-none rounded-full flex justify-between items-center"></div>
        <div className="absolute w-full z-60" style={{ opacity: 0 }}>
          <input
            id="large-range"
            type="range"
            value={value}
            min={min}
            max={max}
            step={step}
            onChange={handleChange}
            className="w-full h-[50px] font-medium text-center pl-10 pr-10 leading-none rounded-full flex justify-between items-center cursor-pointer"
          />
        </div>
        <div
          className="bg-[#D3B871] text-xl  h-[50px] w-[100%] font-medium text-blue-100 text-center pl-10 pr-0.5 cursor-pointer leading-none rounded-full flex justify-between items-center"
          style={{ width: `${value}%` }}
        >
          {' '}
          <div className="text-white">{value * 10} days</div>
          <div>
            <button
              type="button"
              className="text-white mt-1.5 bg-white font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 "
            >
              <Image src="svg/double-arrow.svg" width={50} height={20} alt="double arrow"/>
            </button>
          </div>
        </div>
      </div>
      <div className="text-4xl text-[#D3B871] font-bold">
        <p>{percentage.toFixed(2)} %</p>
      </div>
    </div>
  );
};

export default RangeSlider;
