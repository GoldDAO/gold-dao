import { useEffect } from "react";
import {
  GLDT_LEDGER_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
  REVERSE_GLDT_TX_FEE,
  SWAP_CANISTER_ID,
} from "@constants";
import { useAuth } from "@auth/index";
import useApprove from "@services/ledger/hooks/useApprove";
import useSwapTokensForNFT from "@shared/hooks/useSwapTokensForNFT";
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
  const approve = useApprove(GLDT_LEDGER_CANISTER_ID, authenticatedAgent);
  const swapTokensForNFT = useSwapTokensForNFT(authenticatedAgent, {
    canister_id: canisterId,
    name,
  });

  const handleApprove = () => {
    approve.mutate({
      amount: BigInt(value * GLDT_VALUE_1G_NFT * 10 ** 8 + REVERSE_GLDT_TX_FEE),
      spender: {
        owner: SWAP_CANISTER_ID,
        subaccount: nft.id_byte_array,
      },
    });
  };
  const handleSwapToken = () => {
    swapTokensForNFT.mutate({
      nft,
    });
  };

  const handleRetryApprove = () => {
    approve.reset();
    handleApprove();
  };

  const handleRetrySwapToken = () => {
    swapTokensForNFT.reset();
    handleSwapToken();
  };

  useEffect(() => {
    if (approve.isIdle && swapTokensForNFT.isIdle) {
      handleApprove();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [approve.isIdle, swapTokensForNFT.isIdle, canisterId]);

  useEffect(() => {
    if (swapTokensForNFT.isIdle) {
      handleSwapToken();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swapTokensForNFT.isIdle, canisterId]);

  useEffect(() => {
    return () => {
      approve.reset();
      swapTokensForNFT.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          {approve.status !== "success" && (
            <>
              <MutationStatusIcons status={approve.status} />
              <div>{nft.id_string} - Ledger approve</div>
            </>
          )}
          {approve.status === "success" && (
            <>
              <MutationStatusIcons status={swapTokensForNFT.status} />
              <div>{nft.id_string} - Swap Token for NFT</div>
            </>
          )}
        </div>
        {approve.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleRetryApprove}>
              Retry
            </BtnPrimary>
          </div>
        )}
        {swapTokensForNFT.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleRetrySwapToken}>
              Retry
            </BtnPrimary>
          </div>
        )}
      </div>
    </div>
  );
};

export default CollectionItem;
