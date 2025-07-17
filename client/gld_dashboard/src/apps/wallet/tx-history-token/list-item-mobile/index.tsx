import { useAtomValue } from "jotai";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { Transaction } from "@services/ledger-index/utils/interfaces";
import useFetchLedgerDecimals from "@shared/hooks/useFetchLedgerDecimals";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import Address from "@components/strings/Address";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import CopyAddressBtn from "@wallet/tx-history-token/list-item-mobile/CopyAddressBtn";

const ListItemMobile = ({
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
    return address ? (
      <Address enableTooltip={false} enableCopyToClipboard={false} size="sm">
        {address}
      </Address>
    ) : (
      "N/A"
    );
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
          "p-2 border border-border rounded-xl",
          "flex items-start justify-between"
        )}
      >
        <div className="flex gap-4">
          <div className="w-13 h-13 flex justify-center items-center border border-gold/5 bg-gold/10 rounded-lg">
            <div className="text-xs text-copper">{tx.kind}</div>
          </div>

          <div>
            <div className="flex items-center gap-2 mb-1">
              <div className="text-sm">
                {tx.is_credit ? renderAddress(tx.from) : renderAddress(tx.to)}
              </div>
              <CopyAddressBtn from={tx.from ?? "N/A"} to={tx.to ?? "N/A"} />
            </div>
            <div className="text-content/60 text-xs">{tx.timestamp}</div>
          </div>
        </div>

        <div>
          <div className="text-right text-sm">{renderAmount(tx.amount)}</div>
          <div className="text-content/60 text-xs text-right">
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

export default ListItemMobile;
