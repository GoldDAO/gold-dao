import clsx from "clsx";
import { useAtomValue } from "jotai";

import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "../../atoms";

import { Transaction } from "@services/ledger-index/utils/interfaces";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

const ListItem = ({ tx }: { tx: Transaction }) => {
  const { unauthenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: token.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  return (
    <div
      className={clsx(
        "p-4 border border-border rounded-lg",
        "flex items-start justify-between"
      )}
    >
      <div className="flex items-center gap-4">
        <div className="w-24 flex justify-center px-4 py-3 border border-primary/5 bg-primary/10 text-primary rounded-lg">
          <div>{tx.kind}</div>
        </div>
        <div className="text-sm">
          <div>{tx.to}</div>
          <div className="text-content/60">{tx.timestamp}</div>
        </div>
      </div>
      <div>
        <div>
          {decimals.isSuccess ? (
            <TokenValueToLocaleString
              value={tx.amount as bigint}
              tokenDecimals={decimals.data}
            />
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ListItem;
