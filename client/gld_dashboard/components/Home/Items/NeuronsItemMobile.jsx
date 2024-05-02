"use client";

import { arrayToHex, elapsedTime, formatDateFromSeconds } from "../../../utils/functions";

import { parseNumbers } from "../../../utils/parsers";
import { useState } from "react";

const NeuronsItemMobile = ({ neuron }) => {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <div className="border-t border-[#C6C6C6]">
      <div className="flex items-center cursor-pointer " onClick={() => setIsOpen(!isOpen)}>
        <div
          className={`w-10 h-10 flex justify-center items-center  ${isOpen ? " rotate-180" : "rotate-0"}`}
        >
          <svg
            width="10"
            height="5"
            viewBox="0 0 14 8"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M1 1L7 7L13 1"
              stroke="black"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>

        <div className="w-full  sm:w-[100%] pl-3 grid grid-cols-7 gap-5 text-center py-6  border-[#C6C6C6]">
          <div className=" flex justify-start items-center">
            {" "}
            <p className="text-[#C6C6C6] text-[12px] underline">
              {`${neuron.id && arrayToHex(neuron.id)?.replace(/0/g, "")?.slice(0, 3)}
              ...
              ${neuron.id && arrayToHex(neuron.id)?.replace(/0/g, "")?.slice(3, 6)}`}
            </p>
          </div>

          <div className="flex justify-start items-center">
            <span className="px-4 text-white text-[10px] font-bold rounded-[60px]  h-[32px] bg-[#C6C6C6] flex items-center">
              {neuron.dissolving}
            </span>
          </div>

          <div className="flex justify-start items-center">
            {" "}
            <p className="text-[12px]">
              <span className="font-bold whitespace-nowrap truncate">
                {parseNumbers(neuron.stakedAmount)}
              </span>
              GLDGov
            </p>
          </div>
          <div className=" flex justify-start items-center text-[12px]">
            {" "}
            <p className="font-bold">{(neuron.maturity / 10e7).toFixed(2) || 0}</p>
          </div>
          <div className=" flex justify-start items-center text-[12px]">
            <p className="font-bold ">
              {elapsedTime(neuron.dissolveDelay) === "a minute"
                ? "1 minute"
                : elapsedTime(neuron.dissolveDelay)}
            </p>
          </div>
          <div className="flex justify-start items-center text-[12px]">
            {" "}
            <p className="font-bold  ">{elapsedTime(neuron.age)}</p>
          </div>
          <div className="flex justify-start items-center text-[12px]  ">
            {" "}
            <p className="font-bold ">{neuron.votingPower}</p>
          </div>
        </div>
      </div>

      {isOpen && (
        <section
          className="flex justify-start w-[100%] gap-10 px-20 py-5"
          style={{ transition: "height 0.3s ease" }}
        >
          <div>
            <p className="text-2xs font-bold text-[#D3B871]">Date Created</p>
            <p className="text-2xs font-bold mt-2">
              {formatDateFromSeconds(neuron.dateCreated)},{" "}
              {elapsedTime(Math.round(new Date().getTime() / 1000) - neuron.dateCreated)}
            </p>
          </div>
          <div>
            <p className="text-2xs font-bold text-[#D3B871]">Auto-Stake Maturity</p>
            <p className="text-2xs font-bold mt-2">{neuron.autoStakeMaturity ? "Yes" : "No"}</p>
          </div>
          <div>
            <p className="text-2xs font-bold text-[#D3B871]">Dissolve Delay Bonus</p>
            <p className="text-2xs font-bold mt-2">
              {(neuron.dissolveDelayBonus * 100).toFixed(2)} %
            </p>
          </div>
          <div>
            <p className="text-2xs font-bold text-[#D3B871]">Age Bonus</p>
            <p className="text-2xs font-bold mt-2">{(neuron.ageBonus * 100).toFixed(2)} %</p>
          </div>
          <div>
            <p className="text-2xs font-bold text-[#D3B871]">Total Bonus</p>
            <p className="text-2xs font-bold mt-2">
              {((neuron.totalBonus - 1) * 100).toFixed(2)} %
            </p>
          </div>
        </section>
      )}
    </div>
  );
};

export default NeuronsItemMobile;
