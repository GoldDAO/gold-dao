import { ArrowRightIcon } from "@heroicons/react/20/solid";

import NftWeight from "./NftWeight";
import GldtAmount from "./GldtAmount";

import { SwapData } from "@canisters/gldt_swap/interfaces";

const Swap = ({ data }: { data: SwapData }) => {
  return (
    <div className="border border-border rounded-xl bg-surface p-4 md:p-6">
      <>
        <div className="mb-4 font-semibold">{data.label}</div>
        <div className="sm:flex justify-between items-center sm:w-full grid grid-cols-1 gap-6">
          {data.type === "forward" && (
            <NftWeight className="flex-1" nftWeight={data.send_value} />
          )}
          {data.type === "reverse" && (
            <GldtAmount className="flex-1" gldtAmount={data.send_value} />
          )}
          <div className="bg-content mx-auto sm:mx-0 text-background rounded-full p-2">
            <ArrowRightIcon
              className="rotate-90 sm:rotate-0"
              height={32}
              width={32}
            />
          </div>
          {data.type === "forward" && (
            <GldtAmount className="flex-1" gldtAmount={data.receive_value} />
          )}
          {data.type === "reverse" && (
            <NftWeight className="flex-1" nftWeight={data.receive_value} />
          )}
        </div>
      </>
    </div>
  );
};

export default Swap;
