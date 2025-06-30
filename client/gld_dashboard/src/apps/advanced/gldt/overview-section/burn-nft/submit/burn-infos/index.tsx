import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const BurnInfos = ({
  totalGLDTSelected,
  totalNFTSelected,
}: {
  totalGLDTSelected: number;
  totalNFTSelected: number;
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
    </div>
  );
};

export default BurnInfos;
