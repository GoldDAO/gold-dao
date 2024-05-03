import Image from "next/image";
import { useState } from "react";

const CanistersHeader = () => {
  const [hovered, setHovered] = useState({ disabled: false });

  const handleHover = ({ disabled, index }) => setHovered({ ...hovered, disabled, index });

  const headerItems = [
    { label: "ID", infoSrc: "/svg/info.svg", tooltip: "The unique identifier of the neuron." },
    {
      label: "Type",
      infoSrc: "/svg/info.svg",
      tooltip:
        "The type of SNS canister (i.e., Archive, Dapp, Governance, Index, Ledger, Root, or Swap).",
    },
    {
      label: "Cycles Balance",
      infoSrc: "/svg/info.svg",
      tooltip: "The current cycles balance of the canister in trillion cycles.",
    },
    {
      label: "Freezing Threshold Cycles",
      infoSrc: "/svg/info.svg",
      tooltip:
        "The freezing threshold of the canister in trillion cycles, given its current memory footprint, compute and storage cost, memory and compute allocation, and freezing threshold in seconds setting.",
    },
    {
      label: "Idle Cycles Burned Per Day",
      infoSrc: "/svg/info.svg",
      tooltip:
        "The idle resource consumption rate of the canister in cycles per day, given its current memory footprint, compute and storage cost, and memory and compute allocation.",
    },
    {
      label: "Memory Size",
      infoSrc: "/svg/info.svg",
      tooltip:
        "The total memory size of the canister in bytes, including execution memory (i.e., heap, stable, globals, and Wasm) and for canisters in application subnets, system state memory (i.e., memory used by canister messages).",
    },
    {
      label: "Status",
      infoSrc: "/svg/info.svg",
      tooltip:
        "The status of the canister. Indicates whether the canister is running, stopping, or stopped.",
    },
  ];

  return (
    <thead className="border-t-[0.5px] border-DarkGrey flex items-center justify-around py-[20px] px-4">
      <div className={`w-10 h-10 flex justify-center items-center`}></div>

      <tr className="grid grid-cols-7 w-[1200px] sm:w-[100%] pl-3 gap-5">
        {headerItems.map((item, index) => (
          <th key={index} className="w-full flex items-center justify-start gap-1">
            <p className="text-xs font-bold text-[#D3B871] text-start">{item.label}</p>
            <div
              className="hidden sm:flex"
              onMouseEnter={() => handleHover({ disabled: true, index })}
              onMouseLeave={() => setHovered({ disabled: false, index: null })}
            >
              <Image src={"/svg/info.svg"} className="relative" alt="info" height={20} width={20} />

              <p
                className={`${hovered.disabled && hovered.index === index ? "text-sm tracking-tighter leading-tight z-30 absolute transform top-0 bg-black text-white px-4 py-2 rounded-lg" : "hidden"} max-w-80`}
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

export default CanistersHeader;
