import { BlockTx } from "@services/nft/utils/interfaces";

const NFTValue = ({ tx }: { tx: BlockTx }) => {
  // const { nft_id, created_at, from, to, type } = tx;
  console.log(tx);
  return (
    <div>
      <div className="text-right text-lg">Value NFT</div>
      <div className="text-content/60 text-sm text-right">Infos</div>
    </div>
  );
};

export default NFTValue;
