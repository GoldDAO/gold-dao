import { useEffect } from "react";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm } from "react-hook-form";
import Dialog from "@shared/ui/dialog/Dialog";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import {
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import { NFTCollections } from "@shared/utils/nfts";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import SwitchTransfer from "@shared/components/switch/SwitchTransfer";
import { NFTCollectionSection } from "@wallet/nft/shared/NFTSelectionGrid";

const Select = () => {
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);

  const {
    register,
    handleSubmit,
    setValue,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
    defaultValues: {
      recipient_address: "",
    },
  });

  useEffect(() => {
    if (transferNFTState.send_receive_address !== "") {
      setValue("recipient_address", transferNFTState.send_receive_address);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    transferNFTState.send_receive_address,
    transferNFTState.is_open_send_dialog_select,
  ]);

  const isValidRecipientAddress = (value: string) => {
    try {
      decodeIcrcAccount(value);
      return true;
    } catch (err) {
      console.error(err);
      return false;
    }
  };

  const handleOnSubmit = (data: FieldValues) => {
    dispatchTransferNFTState({
      type: "OPEN_SEND_DIALOG_CONFIRM",
      value: data.recipient_address as string,
    });
  };

  const handleClose = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchTransferNFTState({ type: "RESET" });
  };

  const handleChangeTab = (value: "send" | "receive") => {
    dispatchTransferNFTState({ type: "SET_TAB", value });
  };

  return (
    <Dialog
      open={transferNFTState.is_open_send_dialog_select}
      handleOnClose={handleClose}
      size="xl"
    >
      <SwitchTransfer
        className="flex justify-center mb-12"
        value={transferNFTState.transfer_tab}
        handleChange={handleChangeTab}
      />
      <form onSubmit={handleSubmit(handleOnSubmit)} className="mt-8">
        <div className="mb-6">
          <div className="text-lg font-semibold mb-4">
            Select NFTs to Transfer
          </div>
          <div className="max-h-[500px] overflow-y-auto">
            {NFTCollections.map((collection) => (
              <NFTCollectionSection
                key={collection.name}
                collectionName={collection.name}
                fetchType="user"
              />
            ))}
          </div>
        </div>

        <input
          id="recipient_address"
          type="text"
          autoComplete="off"
          placeholder="Principal ID"
          className={clsx(
            "w-full border border-border outline-none focus:outline-none focus:ring-0 p-4 rounded-xl bg-surface-primary",
            "text-sm font-semibold",
            "placeholder:text-content/60 placeholder:text-sm placeholder:font-semibold"
          )}
          {...register("recipient_address", {
            pattern: /[0-9.]/,
            required: "Recipient address is required",
            validate: {
              isValidRecipientAddress: (v) =>
                isValidRecipientAddress(v) || "Invalid recipient address",
            },
          })}
        />
        {errors && (
          <p className="text-danger text-sm font-semibold mt-1 ml-2">
            {typeof errors?.recipient_address?.message === "string" &&
              errors.recipient_address.message}
          </p>
        )}

        <BtnPrimary
          type="submit"
          disabled={!isValid || totalNFTSelected === 0}
          className="mt-8 w-full"
        >
          Transfer ({totalNFTSelected} selected)
        </BtnPrimary>
      </form>
    </Dialog>
  );
};

export default Select;
