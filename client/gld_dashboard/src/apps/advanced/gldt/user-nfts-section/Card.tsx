import { NFT } from "@services/nft/utils/interfaces";

const Card = ({ nft }: { nft: NFT }) => {
  return (
    <div className="border border-border rounded-xl p-4">
      {nft.img_preview && (
        <img
          src={nft.img_preview}
          alt={nft.name}
          className="w-full h-auto rounded-lg mb-2"
        />
      )}
      {nft.name && <div className="text-content">{nft.name}</div>}
    </div>
  );
};

export default Card;
