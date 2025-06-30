import { TokensList, GLDT_INDEX } from "@wallet/shared/utils";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";

const WalletList = () => {
  return (
    <div className="flex flex-col gap-2">
      <ListItemToken
        token={TokensList[GLDT_INDEX]}
        key={TokensList[GLDT_INDEX].id}
      />
      <ListItemNFT />
      {TokensList.slice(1).map((token) => (
        <ListItemToken token={token} key={token.id} />
      ))}
    </div>
  );
};

export default WalletList;
