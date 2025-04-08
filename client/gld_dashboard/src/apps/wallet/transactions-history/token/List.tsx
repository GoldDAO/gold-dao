import { Fragment, useEffect } from "react";
import { useInView } from "react-intersection-observer";
import { useAtomValue } from "jotai";

import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "../../atoms";
import useFetchAccountTransactions from "@services/ledger-index/hooks/useFetchAccountTransactions";

import ListItem from "./ListItem";

const List = () => {
  const { authenticatedAgent, isConnected } = useAuth();
  const { ref, inView } = useInView();
  const token = useAtomValue(TokenSelectedAtom);

  const { status, data, isFetchingNextPage, fetchNextPage, hasNextPage } =
    useFetchAccountTransactions(
      token.canister_id_ledger_index,
      authenticatedAgent,
      {
        account:
          "4lxgi-y7rlh-onvu4-jtszk-z67wq-ldekw-rfsp3-yxrjy-dgwsl-zn6tl-eqe", // principalId
        enabled: !!authenticatedAgent && isConnected,
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
    <div className="flex flex-col lg:flex-grow lg:h-full gap-2 lg:overflow-y-auto lg:pr-4">
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
