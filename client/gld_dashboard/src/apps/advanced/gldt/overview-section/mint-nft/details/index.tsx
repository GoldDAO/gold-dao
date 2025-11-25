import { useAtom } from "jotai";
import { LoaderSpin } from "@components/index";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import Collection from "./Collection";
import BtnPrimary from "@shared/ui/button/HorizontalButton";

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

  const collections = [
    swapNFT.collections["1G"],
    swapNFT.collections["10G"],
    swapNFT.collections["100G"],
    swapNFT.collections["1KG"],
  ].filter((collection) => collection.total_count_selected > 0);

  return (
    <div>
      <div className="grid grid-cols-1 gap-4 my-8">
        {collections.map((collection) => (
          <Collection key={collection.name} collection={collection} />
        ))}
      </div>

      <BtnPrimary onClick={handleClose} className="w-full">
        Close
      </BtnPrimary>
    </div>
  );
};

export default Details;
