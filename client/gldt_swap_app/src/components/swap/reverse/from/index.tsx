import { useNft } from "@context/index";

import Card from "@components/shared/card/Base";

// import { useIcrc1BalanceOf } from "@hooks/gldt_ledger";
// import { useWallet } from "@amerej/artemis-react";

const ReverseSwapFrom = () => {
  // const { principalId } = useWallet();
  const { getSelectedTotalGLDTNFTs } = useNft();
  const totalGLDTtoSwap = getSelectedTotalGLDTNFTs();
  // const balance = useIcrc1BalanceOf({ owner: principalId as string });

  return (
    <div className="">
      <Card>
        <div className="flex justify-center items-center">
          <div className="font-semibold text-content/40">
            {totalGLDTtoSwap} GLDT
          </div>
        </div>
      </Card>
    </div>
  );
};

export default ReverseSwapFrom;
