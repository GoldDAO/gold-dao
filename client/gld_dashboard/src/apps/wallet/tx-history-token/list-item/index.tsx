import { useAtomValue } from "jotai";
import clsx from "clsx";
import Icon from "@shared/ui/icons";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { Transaction } from "@services/ledger-index/utils/interfaces";
import useFetchLedgerDecimals from "@shared/hooks/useFetchLedgerDecimals";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import Address from "@components/strings/Address";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListItem = ({
  tx,
  className,
}: {
  tx: Transaction;
  className?: string;
}) => {
  const { unauthenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);

  const decimals = useFetchLedgerDecimals(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const price = useFetchTokenPrice(unauthenticatedAgent, {
    from: token.name,
    from_canister_id: token.canister_id,
    amount: tx.amount ?? 0n,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const renderAddress = (address: string | undefined) => {
    return address ? <Address size="lg">{address}</Address> : "N/A";
  };

  const renderAmount = (amount: bigint | undefined) => {
    if (!decimals.isSuccess) return <div>Loading...</div>;
    if (tx.is_credit) {
      return (
        <div className="text-success">
          +
          <E8sToLocaleString
            value={amount as bigint}
            tokenDecimals={decimals.data}
          />{" "}
          {token.name}
        </div>
      );
    } else {
      return (
        <div className="text-danger">
          -
          <E8sToLocaleString
            value={amount as bigint}
            tokenDecimals={decimals.data}
          />{" "}
          {token.name}
        </div>
      );
    }
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
            {tx.kind}
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
            <div className="text-content/60 text-sm">{tx.timestamp}</div>
          </div>
        </div>
        <div>
          <div className="text-right text-lg">{renderAmount(tx.amount)}</div>
          <div className="text-content/60 text-sm text-right">
            {price.isSuccess ? (
              <>
                $
                <NumberToLocaleString value={price.data.amount_usd} />
              </>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ListItem;
