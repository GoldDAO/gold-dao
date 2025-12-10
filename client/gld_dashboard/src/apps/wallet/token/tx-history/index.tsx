import { Fragment, useEffect, useMemo } from "react";
import { useInView } from "react-intersection-observer";
import { useAtomValue } from "jotai";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { Transaction } from "@services/ledger_indexer/utils/interfaces";
import useFetchAccountTransactions from "@shared/hooks/useFetchAccountTransactions";
import ListItem from "@wallet/token/tx-history/list-item";
import ListItemMobile from "@wallet/token/tx-history/list-item-mobile";

const List = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const { ref, inView } = useInView();
  const token = useAtomValue(TokenSelectedAtom);

  const txs = useFetchAccountTransactions(
    token.canister_id_ledger_index,
    unauthenticatedAgent,
    {
      account: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
      ledger: token.name,
    }
  );

  const data = useMemo<Transaction[]>(
    () => (txs.data ? txs.data.pages.flatMap((page) => page.data) : []),
    [txs]
  );

  useEffect(() => {
    if (inView) {
      txs.fetchNextPage();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [inView]);

  if (txs.isPending) {
    return <div>Loading...</div>;
  }

  if (txs.isError) {
    return <div>Error</div>;
  }

  if (txs.isSuccess && data.length === 0) {
    return (
      <div className="p-4 bg-surface-primary flex justify-center border border-border rounded-xl">
        <div>No transactions found</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2">
      {data.map((tx) => (
        <Fragment key={tx.index}>
          <ListItem className="hidden xl:block" tx={tx} />
          <ListItemMobile className="block xl:hidden" tx={tx} />
        </Fragment>
      ))}
      <div ref={ref}></div>
      <div className="p-4 flex justify-center">
        {txs.isFetchingNextPage ? (
          <div>Loading...</div>
        ) : (
          <div>{!txs.hasNextPage && <div>No more transactions found</div>}</div>
        )}
      </div>
    </div>
  );
};

export default List;
