import { useAtom } from "jotai";
import { useEffect } from "react";
import { useAuth } from "@auth/index";
import { Dialog } from "@components/index";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { IdNFT } from "@services/gld_nft/utils/interfaces";
import {
  CollectionNFT,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import useApprove from "@services/ledger/hooks/useApprove";
import useTransferNFT from "@shared/hooks/useTransferNFT";
import { OGY_LEDGER_CANISTER_ID } from "@constants";
import useFetchTransferFeeNFT from "@services/gld_nft/hooks/useFetchTransferFee";
import useFetchTransferFeeLedger from "@services/ledger/hooks/useFetchTransferFee";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const NFTItem = ({
  nft,
  nftCollectionCanisterId,
  nftCollectionName,
  approveStatus,
}: {
  nft: IdNFT;
  nftCollectionCanisterId: string;
  nftCollectionName: string;
  approveStatus: "pending" | "error" | "success" | "idle";
}) => {
  const { authenticatedAgent } = useAuth();
  const [transferNFTState] = useAtom(TransferNFTStateReducerAtom);
  const transfer = useTransferNFT(
    nftCollectionCanisterId,
    nftCollectionName,
    authenticatedAgent
  );

  const handleTransfer = () => {
    transfer.mutate({
      to: transferNFTState.send_receive_address,
      token_id: nft.id_bigint,
    });
  };

  useEffect(() => {
    if (approveStatus === "success") {
      handleTransfer();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [approveStatus]);

  useEffect(() => {
    return () => {
      transfer.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleOnRetry = () => {
    transfer.reset();
    handleTransfer();
  };

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          {approveStatus !== "success" && (
            <>
              <MutationStatusIcons status={approveStatus} />
              <div>Approve {nft.id_string} NFT transfer amount</div>
            </>
          )}
          {approveStatus === "success" && (
            <>
              <MutationStatusIcons status={transfer.status} />
              <div>Transfer {nft.id_string} NFT</div>
            </>
          )}
        </div>
        {transfer.isError && (
          <div>
            <BtnPrimary size="sm" onClick={handleOnRetry}>
              Retry
            </BtnPrimary>
          </div>
        )}
      </div>
    </div>
  );
};

const NFTCollection = ({ collection }: { collection: CollectionNFT }) => {
  const { authenticatedAgent, isConnected, unauthenticatedAgent } = useAuth();
  const approve = useApprove(OGY_LEDGER_CANISTER_ID, authenticatedAgent);

  const nftTransferFee = useFetchTransferFeeNFT(
    collection.canister_id,
    unauthenticatedAgent,
    {
      enabled: isConnected && !!unauthenticatedAgent,
      nft_id: collection.nfts_selected[0].id_bigint,
      nft_id_string: collection.nfts_selected[0].id_string,
    }
  );

  const ledgerTransferFee = useFetchTransferFeeLedger(
    OGY_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "ogy",
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  useEffect(() => {
    if (nftTransferFee.isSuccess && ledgerTransferFee.isSuccess) {
      approve.mutate(
        {
          amount:
            BigInt(collection.nfts_selected.length) *
            (nftTransferFee.data + ledgerTransferFee.data),
          spender: {
            owner: collection.canister_id,
          },
        },
        {
          onSuccess: (res) => {
            console.log("approved");
            console.log(res);
          },
        }
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    nftTransferFee.isSuccess,
    ledgerTransferFee.isSuccess,
    collection.canister_id,
  ]);

  useEffect(() => {
    return () => {
      approve.reset();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="p-4 border border-border rounded-md">
      <div className="text-xl p-2 mb-3">{collection.label} collection</div>
      <div className="flex flex-col gap-2">
        {collection.nfts_selected.map((nft) => (
          <NFTItem
            key={nft.id_string}
            nft={nft}
            nftCollectionCanisterId={collection.canister_id}
            nftCollectionName={collection.name}
            approveStatus={approve.status}
          />
        ))}
      </div>
    </div>
  );
};

const SendNFTDetails = () => {
  const [, dispatchTransferNFTState] = useAtom(TransferNFTStateReducerAtom);
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  const handleOnClose = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchTransferNFTState({ type: "RESET" });
  };

  return (
    <>
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
      <BtnPrimary className="w-full" onClick={handleOnClose}>
        Go to wallet view
      </BtnPrimary>
    </>
  );
};

const Details = () => {
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  const { is_open_send_dialog_details } = transferNFTState;

  const handleOnClose = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchTransferNFTState({ type: "RESET" });
  };

  return (
    <Dialog
      open={is_open_send_dialog_details}
      handleOnClose={handleOnClose}
      title="Send NFT details"
    >
      <SendNFTDetails />
    </Dialog>
  );
};

export default Details;
