import { useEffect } from "react";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm } from "react-hook-form";
import Dialog from "@shared/ui/dialog/Dialog";
import { NFTCollections } from "@shared/utils/nfts";
import UserNFTSelect from "@shared/components/nft-select/UserNFTSelect";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import {
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import SwitchTransfer from "@shared/components/switch/SwitchTransfer";

const Form = () => {
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);

  const {
    register,
    handleSubmit,
    // control,
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
    transferNFTState.is_open_send_dialog_form,
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
      value: data.recipient_address,
    });
  };

  const handleChangeTab = (value: "send" | "receive") => {
    dispatchTransferNFTState({ type: "SET_TAB", value });
  };

  return (
    <>
      <SwitchTransfer
        className="flex justify-center mb-12"
        value={transferNFTState.transfer_tab}
        handleChange={handleChangeTab}
      />
      <form onSubmit={handleSubmit(handleOnSubmit)} className="mt-8">
        <div className="flex flex-col gap-2 mb-4 border border-border p-4 rounded-xl">
          {NFTCollections.map((collection) => (
            <UserNFTSelect key={collection.name} collection={collection.name} />
          ))}
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
          Transfer
        </BtnPrimary>
      </form>
    </>
  );
};

const FormDialog = () => {
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  const handleClose = () => {
    dispatchTransferNFTState({ type: "RESET" });
    dispatchSelectNFTState({ type: "RESET" });
  };

  return (
    <Dialog
      open={transferNFTState.is_open_send_dialog_form}
      handleOnClose={handleClose}
      size="xl"
    >
      <Form />
    </Dialog>
  );
};

export default FormDialog;
