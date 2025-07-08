import { TOKENS, TOKEN_GLDT } from "@shared/utils/tokens";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";

const WalletList = () => {
  return (
    <div className="flex flex-col gap-2">
      <ListItemToken token={TOKEN_GLDT} key={TOKEN_GLDT.id} />
      <ListItemNFT />
      {TOKENS.slice(1).map((token) => (
        <ListItemToken token={token} key={token.id} />
      ))}
    </div>
  );
};

export default WalletList;
