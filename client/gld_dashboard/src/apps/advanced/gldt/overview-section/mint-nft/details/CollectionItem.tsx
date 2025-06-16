import { useEffect } from "react";
import { useAuth } from "@auth/index";
import useSwapNFTForTokens from "@shared/hooks/useSwapNFTForTokens";
import useMarketTransferNFTOrigyn from "@services/gld_nft/hooks/useMarketTransferNFTOrigyn";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { CollectionNameNFT, IdNFT } from "@services/gld_nft/utils/interfaces";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const CollectionItem = ({
  nft,
  canisterId,
  value,
  name,
}: {
  nft: IdNFT;
  canisterId: string;
  value: number;
  name: CollectionNameNFT;
}) => {
  const { authenticatedAgent } = useAuth();
  const swapNFTForTokens = useSwapNFTForTokens(authenticatedAgent, {
    canister_id: canisterId,
    name,
  });
  const marketTransferNFT = useMarketTransferNFTOrigyn(authenticatedAgent, {
    collection_name: canisterId,
  });

  const handleSwapNFT = () => {
    swapNFTForTokens.mutate({
      nft,
    });
  };
  const handleMarketTransfer = () => {
    marketTransferNFT.mutate({
      nft,
      collection_canister_id: canisterId,
      collection_value: value,
    });
  };

  const handleRetrySwapNFT = () => {
    swapNFTForTokens.reset();
    handleSwapNFT();
  };

  const handleRetryMarketTransfer = () => {
    marketTransferNFT.reset();
    handleMarketTransfer();
  };

  useEffect(() => {
    if (swapNFTForTokens.isIdle && marketTransferNFT.isIdle) {
      handleSwapNFT();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapNFTForTokens.isIdle, marketTransferNFT.isIdle, canisterId]);

  useEffect(() => {
    if (swapNFTForTokens.isSuccess && marketTransferNFT.isIdle) {
      handleMarketTransfer();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapNFTForTokens.isSuccess, marketTransferNFT.isIdle, canisterId]);

  useEffect(() => {
    return () => {
      swapNFTForTokens.reset();
      marketTransferNFT.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          {swapNFTForTokens.status !== "success" && (
            <>
              <MutationStatusIcons status={swapNFTForTokens.status} />
              <div>{nft.id_string} - Swap NFT for token</div>
            </>
          )}
          {swapNFTForTokens.status === "success" && (
            <>
              <MutationStatusIcons status={marketTransferNFT.status} />
              <div>{nft.id_string} - Market transfer NFT</div>
            </>
          )}
        </div>
        {swapNFTForTokens.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleRetrySwapNFT}>
              Retry
            </BtnPrimary>
          </div>
        )}
        {marketTransferNFT.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleRetryMarketTransfer}>
              Retry
            </BtnPrimary>
          </div>
        )}
      </div>
    </div>
  );
};

export default CollectionItem;
