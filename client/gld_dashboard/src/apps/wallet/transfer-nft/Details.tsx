import { useAtom } from "jotai";
import { useEffect } from "react";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/Dialog";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { IdNFT } from "@services/gld_nft/utils/interfaces";
import {
  CollectionNFT,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import MutationStatusIcon from "@shared/components/MutationStatusIcon";
import useApprove from "@services/ledger/hooks/useApprove";
import useTransferNFT from "@shared/hooks/useTransferNFT";
import { OGY_LEDGER_CANISTER_ID } from "@constants";
import useFetchNFTTransferFee from "@shared/hooks/useFetchNFTTransferFee";
import BtnPrimary from "@shared/ui/button/BtnPrimary";

const NFTItem = ({
  nft,
  collection,
  txFee,
}: {
  nft: IdNFT;
  collection: CollectionNFT;
  txFee: bigint;
}) => {
  const { authenticatedAgent } = useAuth();
  const [transferNFTState] = useAtom(TransferNFTStateReducerAtom);
  const approve = useApprove(OGY_LEDGER_CANISTER_ID, authenticatedAgent);

  const transfer = useTransferNFT(
    collection.canister_id,
    collection.name,
    authenticatedAgent
  );

  const handleApprove = () => {
    approve.mutate(
      {
        amount: BigInt(collection.nfts_selected.length) * txFee,
        spender: {
          owner: collection.canister_id,
        },
      },
      {
        onSuccess: () => {
          handleTransfer();
        },
      }
    );
  };

  const handleTransfer = () => {
    transfer.mutate({
      to: transferNFTState.send_receive_address,
      token_id: nft.id_bigint,
    });
  };

  useEffect(() => {
    if (approve.isIdle) {
      handleApprove();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [approve.isIdle]);

  useEffect(() => {
    return () => {
      approve.reset();
      transfer.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleOnRetryApprove = () => {
    approve.reset();
    handleApprove();
  };
  const handleOnRetryTransfer = () => {
    approve.reset();
    transfer.reset();
    handleApprove();
  };

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        {(approve.isIdle || approve.isPending) && (
          <div className="flex items-center gap-4">
            <MutationStatusIcon status={approve.status} />
            <div>Approve {nft.id_string} NFT</div>
          </div>
        )}
        {approve.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleOnRetryApprove}>
              Retry
            </BtnPrimary>
          </div>
        )}
        {approve.isSuccess && (
          <div className="flex items-center gap-4">
            <MutationStatusIcon status={transfer.status} />
            <div>Transfer {nft.id_string} NFT</div>
          </div>
        )}
        {transfer.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleOnRetryTransfer}>
              Retry
            </BtnPrimary>
          </div>
        )}
      </div>
    </div>
  );
};

const NFTCollection = ({ collection }: { collection: CollectionNFT }) => {
  const { isConnected, unauthenticatedAgent } = useAuth();

  const txFeeNFT = useFetchNFTTransferFee(
    collection.canister_id,
    unauthenticatedAgent,
    {
      enabled: isConnected && !!unauthenticatedAgent,
      nft_id: collection.nfts_selected[0].id_bigint,
      nft_id_string: collection.nfts_selected[0].id_string,
    }
  );

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="text-xl p-2 mb-3">{collection.label} collection</div>
      <div className="flex flex-col gap-2">
        {collection.nfts_selected.map((nft) =>
          !txFeeNFT.isSuccess ? (
            <div
              key={nft.id_string}
              className="p-4 border border-border rounded-md"
            >
              <div className="flex justify-between items-center">
                <div className="flex items-center gap-4">
                  <MutationStatusIcon status="pending" />
                  <div>Fetching NFT and Ledger fees..</div>
                </div>
              </div>
            </div>
          ) : (
            <NFTItem
              key={nft.id_string}
              nft={nft}
              collection={collection}
              txFee={txFeeNFT.data.amount_e8s}
            />
          )
        )}
      </div>
    </div>
  );
};

const Details = () => {
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  const handleClose = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchTransferNFTState({ type: "RESET" });
  };

  return (
    <Dialog
      open={transferNFTState.is_open_send_dialog_details}
      handleOnClose={handleClose}
      title="Send NFT details"
    >
      <div className="grid grid-cols-1 gap-4 my-8">
        {[
          selectNFTState["1G"],
          selectNFTState["10G"],
          selectNFTState["100G"],
          selectNFTState["1KG"],
        ]
          .filter((collection) => collection.total_count_selected > 0)
          .map((collection) => (
            <NFTCollection key={collection.name} collection={collection} />
          ))}
      </div>
      <BtnPrimary className="w-full" onClick={handleClose}>
        Close
      </BtnPrimary>
    </Dialog>
  );
};

export default Details;
