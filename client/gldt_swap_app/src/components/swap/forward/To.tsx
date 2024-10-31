import { useNft } from "@context/index";

import Card from "@components/shared/card/Base";

const ForwardSwapTo = () => {
  const { getSelectedTotalGLDT } = useNft();
  const totalGLDT = getSelectedTotalGLDT();

  return (
    <div
      className={`border border-border rounded-xl md:p-6 p-4 ${
        !totalGLDT.number ? "opacity-50" : "opacity-100"
      }`}
    >
      <div className="text-gold font-semibold mb-2">To</div>
      <Card>
        <div className="flex justify-center items-center">
          <div className="font-semibold">{totalGLDT.string} GLDT</div>
        </div>
      </Card>
    </div>
  );
};

export default ForwardSwapTo;
