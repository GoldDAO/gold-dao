import { WALLET_LIST } from "@shared/utils/tokens";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";

const WalletList = () => {
  return (
    <div className="flex flex-col gap-2">
      {WALLET_LIST.map((item) =>
        item === "GLDNFT" ? (
          <ListItemNFT key={item} />
        ) : (
          <ListItemToken token={item} key={item.display_name} />
        )
      )}
    </div>
  );
};

export default WalletList;
