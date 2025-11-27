import { useEffect, useMemo } from "react";
import { useInView } from "react-intersection-observer";
import { useAuth } from "@auth/index";
import useFetchUserNftList from "@shared/hooks/useFetchNFTUserList";
import { NFT } from "@services/nft/utils/interfaces";
import Card from "./Card";

const CardList = ({
  canisterId,
  className,
}: {
  canisterId: string;
  className?: string;
}) => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const { ref, inView } = useInView();

  const nfts = useFetchUserNftList(canisterId, {
    agent: unauthenticatedAgent,
    owner: principalId,
    limit: 10,
    enabled: isConnected && !!unauthenticatedAgent && !!canisterId,
  });

  const data = useMemo<NFT[]>(
    () => (nfts.data ? nfts.data.pages.flatMap((page) => page.data) : []),
    [nfts.data]
  );

  useEffect(() => {
    if (inView && nfts.hasNextPage && !nfts.isFetchingNextPage) {
      nfts.fetchNextPage();
    }
  }, [inView, nfts]);

  if (nfts.isPending) {
    return (
      <div className={className}>
        <div>Loading...</div>
      </div>
    );
  }

  if (nfts.isError) {
    return (
      <div className={className}>
        <div>Error</div>
      </div>
    );
  }

  if (nfts.isSuccess && data.length === 0) {
    return (
      <div className={className}>
        <div className="p-4 bg-surface-primary flex justify-center border border-border rounded-xl">
          <div>No NFTs found</div>
        </div>
      </div>
    );
  }

  return (
    <div className={className}>
      <div className="grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {data.map((nft) => (
          <Card key={nft.id.toString()} nft={nft} />
        ))}
        {nfts.hasNextPage && (
          <div ref={ref} className="col-span-full flex justify-center py-4">
            {nfts.isFetchingNextPage && <div>Loading more...</div>}
          </div>
        )}
      </div>
    </div>
  );
};

export default CardList;
