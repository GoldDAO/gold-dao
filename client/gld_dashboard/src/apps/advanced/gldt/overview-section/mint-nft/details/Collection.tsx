/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect } from "react";
import { useAuth } from "@auth/index";
import useApproveNFT from "@shared/hooks/useApproveNFT";
import useSwapNFTForTokens from "@shared/hooks/useSwapNFTForTokens";
import MutationStatusIcon from "@shared/components/MutationStatusIcon";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import { SWAP_CANISTER_ID } from "@constants";
import { CollectionNFT } from "@shared/atoms/NFTStateAtom";

const Collection = ({ collection }: { collection: CollectionNFT }) => {
  const { authenticatedAgent } = useAuth();
  const approve = useApproveNFT(collection.canister_id, authenticatedAgent);
  const swapNFTForTokens = useSwapNFTForTokens(authenticatedAgent, {
    canister_id: collection.canister_id,
  });

  const handleApprove = () => {
    approve.mutate({
      token_ids: collection.nfts_selected.map((nft) => nft.id),
      spender: {
        owner: SWAP_CANISTER_ID,
        subaccount: [],
      },
    });
  };

  const handleSwapNFT = () => {
    swapNFTForTokens.mutate({
      nfts: collection.nfts_selected,
    });
  };

  const handleRetrySwapNFT = () => {
    swapNFTForTokens.reset();
    handleSwapNFT();
  };

  const handleRetryApprove = () => {
    approve.reset();
    handleApprove();
  };

  useEffect(() => {
    if (swapNFTForTokens.isIdle && approve.isIdle) {
      handleApprove();
    }
  }, [swapNFTForTokens.isIdle, approve.isIdle, collection.canister_id]);

  useEffect(() => {
    if (approve.isSuccess && swapNFTForTokens.isIdle) {
      handleSwapNFT();
    }
  }, [approve.isSuccess, swapNFTForTokens.isIdle, collection.canister_id]);

  useEffect(() => {
    return () => {
      approve.reset();
      swapNFTForTokens.reset();
    };
  }, []);

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="text-xl p-2 mb-3">{collection.label} collection</div>
      <div className="p-4 border border-border rounded-md">
        <div className="flex justify-between items-center">
          <div className="flex items-center gap-4">
            {approve.status !== "success" && (
              <>
                <MutationStatusIcon status={approve.status} />
                <div>Approve</div>
              </>
            )}
            {approve.status === "success" && (
              <>
                <MutationStatusIcon status={swapNFTForTokens.status} />
                <div>Minting</div>
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
          {approve.isError && (
            <div>
              <BtnPrimary size="sm" onClick={handleRetryApprove}>
                Retry
              </BtnPrimary>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Collection;
