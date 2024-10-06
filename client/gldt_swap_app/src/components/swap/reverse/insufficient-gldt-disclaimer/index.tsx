import { useNavigate } from "react-router-dom";
import { Button } from "@components/ui";

import { useReverseSwapProceed } from "@context/index";

const InsufficientGLDTDisclaimer = ({ className }: { className?: string }) => {
  const navigate = useNavigate();
  const { state } = useReverseSwapProceed();
  const { totalSwapGLDT, balanceGLDT, countSelectedNfts } = state;

  const handleClickGetGLDT = () => {
    navigate("/swap?view=0&mode=0");
  };

  return (
    <div className={className}>
      <div className="border border-orange-500 bg-orange-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl text-center">
        <div className="mb-6 text-orange-500">
          <div className="font-semibold mb-2">
            You don't have enough GLDT to process
          </div>
          <div>
            To reverse swap the{" "}
            <span className="font-semibold">{countSelectedNfts} GLD NFT</span>{" "}
            selected you need to have at least{" "}
            <span className="font-semibold">{totalSwapGLDT} GLDT</span>.
            <br />
            Your current balance is{" "}
            <span className="font-semibold">{balanceGLDT} GLDT</span>
          </div>
        </div>
        <div>
          <Button onClick={handleClickGetGLDT}>Get GLDT</Button>
        </div>
      </div>
    </div>
  );
};

export default InsufficientGLDTDisclaimer;
