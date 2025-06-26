import { ReactNode, useState } from "react";
import clsx from "clsx";
import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { TokensList, GLDT_INDEX, Token } from "@wallet/shared/utils";
import ListItemToken from "@wallet/wallet-list/list-item-token";
import ListItemNFT from "@wallet/wallet-list/list-item-nft";
import Dialog from "@components/dialogs/Dialog";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { ArrowDown2 } from "iconsax-react";

const TriggerBaseBtn = ({
  children,
  handleOnClick,
}: {
  children: ReactNode;
  handleOnClick: () => void;
}) => {
  return (
    <button
      className={clsx(
        "flex justify-between items-center gap-4",
        "bg-surface-primary dark:bg-surface-secondary hover:bg-primary/10",
        "disabled:opacity-70",
        "px-4 py-2 rounded-full",
        "cursor-pointer disabled:cursor-not-allowed",
        "dark:shadow-surface-secondary/10 shadow-md",
        "min-w-[240px]"
      )}
      onClick={handleOnClick}
    >
      {children}
      <ArrowDown2 size="16" className="" />
    </button>
  );
};

const TriggerNFT = ({ handleOnClick }: { handleOnClick: () => void }) => {
  const { unauthenticatedAgent } = useAuth();
  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent,
  });

  return (
    <TriggerBaseBtn handleOnClick={handleOnClick}>
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-1">
          <Logo name="gld_nft" className="h-6 w-6" />
          <div className="font-semibold text-lg">GLD NFT</div>
        </div>
        <div className="text-sm text-content/60">
          {priceGold.isSuccess ? (
            <>
              1 gram Gold ≈ $
              <NumberToLocaleString value={priceGold.data} />
            </>
          ) : (
            <span>Loading...</span>
          )}
        </div>
      </div>
    </TriggerBaseBtn>
  );
};

const TriggerToken = ({
  handleOnClick,
  token,
}: {
  handleOnClick: () => void;
  token: Token;
}) => {
  const { unauthenticatedAgent } = useAuth();

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: token.id,
    enabled: !!unauthenticatedAgent,
  });

  const price = useFetchTokenPrice(unauthenticatedAgent, {
    from: token.name,
    from_canister_id: token.canisterId,
    amount: BigInt(1 * 10 ** (decimals.data ?? 0)),
    enabled: !!unauthenticatedAgent && decimals.isSuccess,
  });

  return (
    <TriggerBaseBtn handleOnClick={handleOnClick}>
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1">
          <Logo name={token.id} className="h-6 w-6" />
          <div className="font-semibold text-lg">{token.name}</div>
        </div>
        <div className="text-sm text-content/60">
          {price.isSuccess ? (
            <>
              1 {token.name} ≈ $
              <NumberToLocaleString value={price.data.amount_usd} />
            </>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </TriggerBaseBtn>
  );
};

const WalletListMobile = ({ className }: { className?: string }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [searchParams] = useSearchParams();
  const token = useAtomValue(TokenSelectedAtom);

  const handleOnClose = () => {
    setIsOpen(false);
  };

  return (
    <div className={className}>
      {searchParams.get("token") === "nft" ? (
        <TriggerNFT handleOnClick={() => setIsOpen(!isOpen)} />
      ) : (
        <TriggerToken token={token} handleOnClick={() => setIsOpen(!isOpen)} />
      )}
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
