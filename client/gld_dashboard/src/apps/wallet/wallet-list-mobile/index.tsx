import { TokensList, GLDT_INDEX } from "@wallet/shared/utils";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";
import { ReactNode, useState } from "react";
import { Dialog } from "@components/index";
import clsx from "clsx";

const Trigger = ({
  handleOnClick,
  children,
}: {
  handleOnClick: () => void;
  children: ReactNode;
}) => {
  return (
    <button
      className={clsx(
        "text-sm font-semibold",
        "text-primary disabled:text-primary/60",
        "border-1 border-primary/60 disabled:border-primary/60",
        "bg-transparent hover:bg-primary/10",
        "disabled:opacity-70",
        "px-4 py-2 rounded-full",
        "cursor-pointer disabled:cursor-not-allowed"
      )}
      onClick={handleOnClick}
    >
      {children}
    </button>
  );
};

const WalletListMobile = ({ className }: { className?: string }) => {
  const [isOpen, setIsOpen] = useState(false);

  const handleOnClose = () => {
    setIsOpen(false);
  };

  return (
    <div className={className}>
      <Trigger handleOnClick={() => setIsOpen(!isOpen)}>Select Token</Trigger>
      <Dialog open={isOpen} handleOnClose={handleOnClose} closeEnabled={false}>
        <div className="flex flex-col gap-2 pb-4">
          <button onClick={handleOnClose}>
            <ListItemToken
              token={TokensList[GLDT_INDEX]}
              key={TokensList[GLDT_INDEX].id}
            />
          </button>
          <button onClick={handleOnClose}>
            <ListItemNFT />
          </button>
          {TokensList.slice(1).map((token) => (
            <button onClick={handleOnClose} key={token.id}>
              <ListItemToken token={token} />
            </button>
          ))}
        </div>
      </Dialog>
    </div>
  );
};

export default WalletListMobile;
