import { useState } from "react";
import { NFT } from "@services/nft/utils/interfaces";
import Details from "./Details";

const Card = ({ nft }: { nft: NFT }) => {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleCardClick = () => {
    setIsDialogOpen(true);
  };

  const handleCloseDialog = () => {
    setIsDialogOpen(false);
  };

  return (
    <>
      <div
        className="bg-surface-primary shadow-sm rounded-xl p-4 cursor-pointer hover:opacity-80 transition-colors"
        onClick={handleCardClick}
      >
        {nft.img_preview && (
          <img
            src={nft.img_preview}
            alt={nft.name}
            className="w-full h-auto rounded-lg mb-2"
          />
        )}
        {nft.name && <div className="text-content text-center">{nft.name}</div>}
      </div>
      <Details nft={nft} open={isDialogOpen} onClose={handleCloseDialog} />
    </>
  );
};

export default Card;
