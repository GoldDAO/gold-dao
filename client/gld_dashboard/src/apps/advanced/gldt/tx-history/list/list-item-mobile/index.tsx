import clsx from "clsx";
import { BlockTx } from "@services/nft/utils/interfaces";
import Address from "@components/strings/Address";
import CopyAddressBtn from "@wallet/tx-history-token/list-item-mobile/CopyAddressBtn";
import NFTValue from "../shared/NFTValue";

const ListItemMobile = ({
  tx,
  className,
}: {
  tx: BlockTx;
  className?: string;
}) => {
  const renderAddress = (address: string | null) => {
    return address ? (
      <Address enableTooltip={false} enableCopyToClipboard={false} size="sm">
        {address}
      </Address>
    ) : (
      "N/A"
    );
  };

  return (
    <div className={className}>
      <div
        className={clsx(
          "p-2 border border-border rounded-xl",
          "flex items-start justify-between"
        )}
      >
        <div className="flex gap-4">
          <div className="w-13 h-13 flex justify-center items-center border border-gold/5 bg-gold/10 rounded-lg">
            <div className="text-xs text-copper">{tx.type}</div>
          </div>

          <div>
            <div className="flex items-center gap-2 mb-1">
              <div className="text-sm">
                {tx.type === "Transfer"
                  ? renderAddress(tx.from)
                  : renderAddress(tx.to)}
              </div>
              <CopyAddressBtn from={tx.from ?? "N/A"} to={tx.to ?? "N/A"} />
            </div>
            <div className="text-content/60 text-xs">{tx.created_at}</div>
          </div>
        </div>

        <NFTValue />
      </div>
    </div>
  );
};

export default ListItemMobile;
