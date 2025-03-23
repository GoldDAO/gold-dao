import { useEffect } from "react";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm } from "react-hook-form";

import { Button } from "@components/index";
import UserNFTSelect from "./nft.select/UserNFTSelect";
import { SendStateAtom } from "./atoms";
import {
  CollectionNFT1GAtom,
  CollectionNFT10GAtom,
  CollectionNFT100GAtom,
  CollectionNFT1KGAtom,
} from "@atoms/NFTState";

const TransferNFT = ({ className }: { className?: string }) => {
  const collNFT1GState = useAtomValue(CollectionNFT1GAtom);
  const collNFT10GState = useAtomValue(CollectionNFT10GAtom);
  const collNFT100GState = useAtomValue(CollectionNFT100GAtom);
  const collNFT1KGState = useAtomValue(CollectionNFT1KGAtom);

  const [, setSendState] = useAtom(SendStateAtom);

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
  });

  useEffect(() => {
    setValue(
      "recipient_address",
      "oxh25-vm4xh-tmsig-jsjms-3ra3g-jyyqy-nqb2k-swild-u5hfd-qvmmf-qqe",
      { shouldValidate: true }
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
    setSendState((state) => ({
      ...state,
      receive_address: data.recipient_address,
      is_step_send_form: false,
      is_step_send_confirm: true,
    }));
  };

  return (
    <form onSubmit={handleSubmit(handleOnSubmit)} className={className}>
      <div className="flex flex-col gap-2 mb-8 border border-border p-4 rounded-xl">
        <UserNFTSelect collectionAtom={CollectionNFT1GAtom} />
        <UserNFTSelect collectionAtom={CollectionNFT10GAtom} />
        <UserNFTSelect collectionAtom={CollectionNFT100GAtom} />
        <UserNFTSelect collectionAtom={CollectionNFT1KGAtom} />
      </div>

      <input
        id="recipient_address"
        type="text"
        autoComplete="off"
        placeholder="Principal ID"
        className={clsx(
          "mt-4 w-full border border-border outline-none focus:outline-none focus:ring-0 p-4 rounded-md bg-surface-primary",
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
      <div className="mt-1 h-8 ml-2">
        {errors && (
          <p className="text-red-600 text-sm font-semibold">
            {typeof errors?.recipient_address?.message === "string" &&
              errors.recipient_address.message}
          </p>
        )}
      </div>
      <div className="mt-8">
        <Button
          type="submit"
          disabled={
            !isValid ||
            (!collNFT1GState.totalCountSelected &&
              !collNFT10GState.totalCountSelected &&
              !collNFT100GState.totalCountSelected &&
              !collNFT1KGState.totalCountSelected)
          }
          className="w-full px-6 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
        >
          Transfer
        </Button>
      </div>
    </form>
  );
};

export default TransferNFT;
