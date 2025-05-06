import { Fragment, useEffect } from "react";
import { useInView } from "react-intersection-observer";
import { useAtomValue } from "jotai";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "../atoms";
import { Transaction } from "@services/ledger-index/utils/interfaces";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchAccountTransactions from "@services/ledger-index/hooks/useFetchAccountTransactions";

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

const List = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const { ref, inView } = useInView();
  const token = useAtomValue(TokenSelectedAtom);

  const { status, data, isFetchingNextPage, fetchNextPage, hasNextPage } =
    useFetchAccountTransactions(
      token.canister_id_ledger_index,
      unauthenticatedAgent,
      {
        account: principalId, // "4lxgi-y7rlh-onvu4-jtszk-z67wq-ldekw-rfsp3-yxrjy-dgwsl-zn6tl-eqe"
        enabled: !!unauthenticatedAgent && isConnected,
        ledger: token.id,
      }
    );

  useEffect(() => {
    if (inView) {
      fetchNextPage();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [inView]);

  if (status === "pending") {
    return <div>Loading...</div>;
  }

  if (status === "error") {
    return <div>Error</div>;
  }

  if (status === "success" && data?.pages[0]?.data.length === 0) {
    return (
      <div className="p-4 flex justify-center border border-border rounded-lg">
        <div>No transactions found</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col xl:flex-grow xl:h-full gap-2 xl:overflow-y-auto xl:pr-4">
      {data?.pages.map((page) => (
        <Fragment key={page.cursor_index}>
          {page.data.map((tx) => (
            <Fragment key={tx.index}>
              <ListItem tx={tx} />
            </Fragment>
          ))}
        </Fragment>
      ))}
      <div ref={ref}></div>
      <div className="p-4 flex justify-center">
        {isFetchingNextPage ? (
          <div>Loading...</div>
        ) : (
          <div>{!hasNextPage && <div>No more transactions found</div>}</div>
        )}
      </div>
    </div>
  );
};

export default List;
