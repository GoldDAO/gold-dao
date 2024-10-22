import { useNft } from "@context/index";

import Card from "@components/shared/card/Base";

const ReverseSwapFrom = () => {
  const { getSelectedTotalGLDT, getSelectedTotal } = useNft();
  const totalNFTs = getSelectedTotal();
  const totalGLDTtoSwap = getSelectedTotalGLDT();

  return (
    <div className="">
      <Card>
        <div className="flex justify-center items-center">
          <div className="font-semibold text-content/40">
            {totalGLDTtoSwap + totalNFTs} GLDT
          </div>
        </div>
      </Card>
    </div>
  );
};

export default ReverseSwapFrom;
