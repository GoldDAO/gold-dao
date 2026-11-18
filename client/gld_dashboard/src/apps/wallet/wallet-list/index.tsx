import { useAtomValue } from "jotai";
import { WALLET_LIST } from "@shared/utils/tokens";
import { CustomTokensAtom } from "@wallet/shared/atoms/CustomTokensAtom";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";
import AddCustomTokenBtn from "@wallet/token/custom/add-btn";

const WalletList = () => {
  const customTokens = useAtomValue(CustomTokensAtom);

  return (
    <div className="flex flex-col gap-2">
      {WALLET_LIST.map((item) =>
        item === "GLDNFT" ? (
          <ListItemNFT key={item} />
        ) : (
          <ListItemToken token={item} key={item.display_name} />
        )
      )}
      {customTokens.map((token) => (
        <ListItemToken token={token} key={token.display_name} />
      ))}
      <AddCustomTokenBtn />
    </div>
  );
};

export default WalletList;
