import { Link } from "react-router-dom";
import { Button } from "@components/ui";

import { useReverseSwapProceed } from "@context/index";

const InsufficientGLDTDisclaimer = ({ className }: { className?: string }) => {
  const { state } = useReverseSwapProceed();
  const { totalSwapGLDT, balanceGLDT, countSelectedNfts } = state;
  const linkIcpSwapPairICPGLDT =
    "https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=6c7su-kiaaa-aaaar-qaira-cai";

  return (
    <div className={className}>
      <div className="border border-orange-500 bg-orange-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl text-center">
        <div className="mb-6 text-orange-500">
          <div className="font-semibold mb-2">
            You don't have enough GLDT to process.
          </div>
          <div>
            To reverse swap the{" "}
            <span className="font-semibold">{countSelectedNfts} GLD NFT</span>{" "}
            selected you need to have at least{" "}
            <span className="font-semibold">{totalSwapGLDT} GLDT</span>.
            <br />
            Your current balance is{" "}
            <span className="font-semibold">{balanceGLDT} GLDT.</span>
          </div>
        </div>
        <div>
          <Link
            to={linkIcpSwapPairICPGLDT}
            target="_blank"
            rel="noopener noreferrer"
          >
            <Button>Get GLDT</Button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default InsufficientGLDTDisclaimer;
