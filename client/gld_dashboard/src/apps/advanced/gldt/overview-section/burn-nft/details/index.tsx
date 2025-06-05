import { useAtom } from "jotai";
import { Button, LoaderSpin } from "@components/index";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import Collection from "./Collection";

const Details = () => {
  const [swapNFT, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);

  const handleClose = () => {
    dispatchSwapNFT({ type: "RESET" });
  };

  if (!swapNFT.collections) {
    return (
      <div className="flex items-center justify-center my-8">
        <LoaderSpin />
      </div>
    );
  }

  return (
    <div>
      <div className="grid grid-cols-1 gap-4 my-8">
        {[
          swapNFT.collections["1G"],
          swapNFT.collections["10G"],
          swapNFT.collections["100G"],
          swapNFT.collections["1KG"],
        ]
          .filter((collection) => collection.total_count_selected > 0)
          .map((collection) => (
            <Collection key={collection.name} collection={collection} />
          ))}
      </div>

      <Button
        onClick={handleClose}
        disabled={false}
        className="w-full px-6 py-3 bg-secondary text-white font-medium rounded-md"
      >
        Close
      </Button>
    </div>
  );
};

export default Details;
