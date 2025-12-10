import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const BurnInfos = ({
  totalGLDTSelected,
  totalNFTSelected,
  totalCollectionSelected,
}: {
  totalGLDTSelected: number;
  totalNFTSelected: number;
  totalCollectionSelected: number;
}) => {
  return (
    <div className="text-sm">
      You are burning{" "}
      <span className="text-copper font-semibold">
        <NumberToLocaleString value={totalGLDTSelected} /> GLDT
      </span>{" "}
      and will receive{" "}
      <span className="text-copper font-semibold">
        {totalNFTSelected} GLD NFTs
      </span>
      .
      <br />
      For this, you are charged a fee of{" "}
      <span className="text-copper font-semibold">
        {totalNFTSelected} x 1 GLDT
      </span>{" "}
      which will go to the Gold DAO treasury.
      <br />
      Approving selected collections costs{" "}
      <span className="text-copper font-semibold">
        {totalCollectionSelected} x 0.1 GLDT
      </span>
      .
    </div>
  );
};

export default BurnInfos;
