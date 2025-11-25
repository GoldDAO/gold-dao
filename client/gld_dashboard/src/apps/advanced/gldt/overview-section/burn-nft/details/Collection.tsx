/* eslint-disable react-hooks/exhaustive-deps */
import { useEffect } from "react";
import { useAuth } from "@auth/index";
import useApprove from "@shared/hooks/useApproveLedger";
import useSwapTokensForNFT from "@shared/hooks/useSwapTokensForNFT";
import MutationStatusIcon from "@shared/components/MutationStatusIcon";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import {
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
  REVERSE_GLDT_TX_FEE,
} from "@constants";
import { CollectionNFT } from "@shared/atoms/NFTStateAtom";

const Collection = ({ collection }: { collection: CollectionNFT }) => {
  const { authenticatedAgent } = useAuth();
  const approve = useApprove(GLDT_LEDGER_CANISTER_ID, authenticatedAgent);
  const swapTokensForNFT = useSwapTokensForNFT(authenticatedAgent, {
    canister_id: collection.canister_id,
  });

  const handleApprove = () => {
    approve.mutate({
      amount: collection.nfts_selected
        .map(() =>
          BigInt(
            collection.value * GLDT_VALUE_1G_NFT * 10 ** 8 + REVERSE_GLDT_TX_FEE
          )
        )
        .reduce((a, b) => a + b, 0n),
      spender: {
        owner: SWAP_CANISTER_ID,
      },
    });
  };

  const handleSwapNFT = () => {
    swapTokensForNFT.mutate({
      nfts: collection.nfts_selected,
    });
  };

  const handleRetrySwapNFT = () => {
    swapTokensForNFT.reset();
    handleSwapNFT();
  };

  const handleRetryApprove = () => {
    approve.reset();
    handleApprove();
  };

  useEffect(() => {
    if (swapTokensForNFT.isIdle && approve.isIdle) {
      handleApprove();
    }
  }, [swapTokensForNFT.isIdle, approve.isIdle, collection.canister_id]);

  useEffect(() => {
    if (approve.isSuccess && swapTokensForNFT.isIdle) {
      handleSwapNFT();
    }
  }, [approve.isSuccess, swapTokensForNFT.isIdle, collection.canister_id]);

  useEffect(() => {
    return () => {
      approve.reset();
      swapTokensForNFT.reset();
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
                <MutationStatusIcon status={swapTokensForNFT.status} />
                <div>Burning</div>
              </>
            )}
          </div>
          {swapTokensForNFT.isError && (
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
