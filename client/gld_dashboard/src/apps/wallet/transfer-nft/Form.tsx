import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm } from "react-hook-form";
import { NFTCollections } from "@shared/utils/nfts";
import { Button } from "@components/index";
import UserNFTSelect from "@shared/components/nft-select/UserNFTSelect";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import { IsOneOrMoreSelectedNFTAtom } from "@shared/atoms/NFTStateAtom";

const Form = ({ className }: { className?: string }) => {
  const [, dispatchTransferNFTState] = useAtom(TransferNFTStateReducerAtom);
  const IsOneOrMoreSelectedNFT = useAtomValue(IsOneOrMoreSelectedNFTAtom);

  const {
    register,
    handleSubmit,
    // control,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });

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
      type: "STEP_SEND_CONFIRM",
      value: data.recipient_address,
    });
  };

  return (
    <form onSubmit={handleSubmit(handleOnSubmit)} className={className}>
      <div className="flex flex-col gap-2 mb-8 border border-border p-4 rounded-xl">
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
          disabled={!isValid || !IsOneOrMoreSelectedNFT}
          className="w-full px-6 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
        >
          Transfer
        </Button>
      </div>
    </form>
  );
};

export default Form;
