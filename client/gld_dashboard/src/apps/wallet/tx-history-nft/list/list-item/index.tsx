import clsx from "clsx";
import Icon from "@shared/ui/icons";
import { BlockTx } from "@services/nft/utils/interfaces";
import Address from "@components/strings/Address";
import NFTValue from "../shared/NFTValue";

const ListItem = ({ tx, className }: { tx: BlockTx; className?: string }) => {
  const renderAddress = (address: string | null) => {
    return address ? <Address size="lg">{address}</Address> : "N/A";
  };

  return (
    <div className={className}>
      <div
        className={clsx(
          "p-4 border border-border rounded-xl",
          "flex justify-between"
        )}
      >
        <div className="flex items-center gap-4">
          <div className="w-24 flex justify-center px-4 py-3 border border-gold/5 bg-gold/10 text-copper text-sm font-semibold rounded-xl">
            {tx.type}
          </div>
          <div className="text-sm">
            <div className="inline-flex items-center gap-2">
              <div className="text-center mb-2 lg:mb-0">
                {renderAddress(tx.from)}
              </div>
              <div className="flex justify-center">
                <Icon.Arrow width={12} className="-rotate-90" />
              </div>
              <div className="text-center">{renderAddress(tx.to)}</div>
            </div>
            <div className="text-content/60 text-sm">{tx.created_at}</div>
          </div>
        </div>
        <NFTValue tx={tx} />
      </div>
    </div>
  );
};

export default ListItem;
