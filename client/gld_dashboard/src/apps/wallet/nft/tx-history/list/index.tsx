import { Fragment, useEffect, useMemo } from "react";
import { useInView } from "react-intersection-observer";
import { useAuth } from "@auth/index";
import { NFTCollection } from "@services/nft/utils/interfaces";
import ListItem from "./list-item";
import ListItemMobile from "./list-item-mobile";
import useFetchNFTBlocks from "@shared/hooks/useFetchNFTBlocks";
import { BlockTx } from "@services/nft/utils/interfaces";

const List = ({ collection }: { collection: NFTCollection }) => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const { ref, inView } = useInView();

  const txs = useFetchNFTBlocks({
    unauthenticated_agent: unauthenticatedAgent,
    canister_id_collection: collection.canisterId,
    canister_id_collection_indexer: collection.canisterIdIndexer,
    limit: 10,
    principal: principalId,
    sort_by: { Descending: null },
    enabled: isConnected && !!unauthenticatedAgent,
  });

  const data = useMemo<BlockTx[]>(
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
        <Fragment key={tx.tx_id}>
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
