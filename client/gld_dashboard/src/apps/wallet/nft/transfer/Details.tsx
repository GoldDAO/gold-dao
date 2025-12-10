import { useAtom } from "jotai";
import { useEffect } from "react";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/Dialog";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { NFT } from "@services/nft/utils/interfaces";
import {
  CollectionNFT,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import useTransferNFT from "@shared/hooks/useTransferNFT";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import MutationStatusIcon from "@shared/components/MutationStatusIcon";

const NFTItem = ({
  nft,
  collection,
}: {
  nft: NFT;
  collection: CollectionNFT;
}) => {
  const { authenticatedAgent } = useAuth();
  const [transferNFTState] = useAtom(TransferNFTStateReducerAtom);

  const transfer = useTransferNFT(collection.canister_id, authenticatedAgent);

  const handleTransfer = () => {
    transfer.mutate({
      to: transferNFTState.send_receive_address,
      token_id: nft.id,
    });
  };

  useEffect(() => {
    if (transfer.isIdle) {
      handleTransfer();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [transfer.isIdle]);

  useEffect(() => {
    return () => {
      transfer.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleOnRetryTransfer = () => {
    transfer.reset();
  };

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        {(transfer.isIdle || transfer.isPending) && (
          <div className="flex items-center gap-4">
            <MutationStatusIcon status={transfer.status} />
            <div>Transfering {nft.name}</div>
          </div>
        )}
        {transfer.isSuccess && (
          <div className="flex items-center gap-4">
            <MutationStatusIcon status={transfer.status} />
            <div>Transfer {nft.name}</div>
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
  return (
    <div className="p-4 border border-border rounded-md">
      <div className="text-xl p-2 mb-3">{collection.label} collection</div>
      <div className="flex flex-col gap-2">
        {collection.nfts_selected.map((nft) => (
          <NFTItem key={nft.id} nft={nft} collection={collection} />
        ))}
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
