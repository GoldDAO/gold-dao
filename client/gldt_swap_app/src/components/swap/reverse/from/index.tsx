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
      {totalNFTs > 0 && (
        <div className="mt-4 text-sm px-4">
          As you are swapping for{" "}
          <span className="font-semibold text-gold">{totalNFTs} GLD NFTs</span>,
          you are charged{" "}
          <span className="font-semibold text-gold">{totalNFTs} x 1 GLDT</span>{" "}
          for the reverse swap.
          <br />
          <span className="font-semibold text-gold">
            {totalGLDTtoSwap} GLDT
          </span>{" "}
          will be burned and{" "}
          <span className="font-semibold text-gold">{totalNFTs} GLDT</span> go
          to Gold DAO treasury.
        </div>
      )}
    </div>
  );
};

export default ReverseSwapFrom;
