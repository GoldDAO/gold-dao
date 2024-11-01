import { useNft } from "@context/index";

import Card from "@components/shared/card/Base";

const ReverseSwapFrom = () => {
  const {
    getSelectedTotalGLDT,
    getSelectedTotalNFTs,
    getSelectedTotalGLDTWithFees,
  } = useNft();
  const totalNFTs = getSelectedTotalNFTs();
  const totalGLDT = getSelectedTotalGLDT();
  const totalGLDTWithFees = getSelectedTotalGLDTWithFees();

  return (
    <div className="">
      <Card>
        <div className="flex justify-center items-center">
          <div className="font-semibold text-content/40">
            {totalGLDTWithFees.string} GLDT
          </div>
        </div>
      </Card>

      {totalNFTs.number > 0 && (
        <div className="mt-4 text-sm px-4">
          You are burning{" "}
          <span className="font-semibold text-gold">
            {totalGLDT.string} GLDT
          </span>{" "}
          and will receive{" "}
          <span className="font-semibold text-gold">
            {totalNFTs.string} GLD NFTs
          </span>
          .
          <br />
          For this, you are charged a fee of{" "}
          <span className="font-semibold text-gold">
            {totalNFTs.string} x 1 GLDT
          </span>{" "}
          which will go to the Gold DAO treasury.
        </div>
      )}
    </div>
  );
};

export default ReverseSwapFrom;
