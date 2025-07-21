import { useEffect } from "react";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useAtom, useAtomValue } from "jotai";
import clsx from "clsx";
import { FieldValues, useForm } from "react-hook-form";
import { useAuth } from "@auth/index";
import Dialog from "@shared/ui/dialog/Dialog";
import { NFTCollections } from "@shared/utils/nfts";
import useFetchNFTTransferFee from "@shared/hooks/useFetchNFTTransferFee";
import UserNFTSelect from "@shared/components/nft-select/UserNFTSelect";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import {
  SelectNFTStateReducerAtom,
  TotalNFTSelectedAtom,
  RandomSelectedNFTIdAtom,
} from "@shared/atoms/NFTStateAtom";
import BtnPrimary from "@shared/ui/button/BtnPrimary";
import SwitchTransfer from "@shared/components/switch/SwitchTransfer";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import { OGY_LEDGER_CANISTER_ID } from "@constants";
import DisclaimerInsufficientOGYFunds from "./disclaimer-insufficient-ogy-funds";
import { Logo } from "@shared/ui/logos";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import BalanceAvailable from "@shared/components/BalanceAvailable";

const Form = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const [transferNFTState, dispatchTransferNFTState] = useAtom(
    TransferNFTStateReducerAtom
  );
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const randomSelectedNFTId = useAtomValue(RandomSelectedNFTIdAtom) || null;

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

  const txFeeNFT = useFetchNFTTransferFee(
    randomSelectedNFTId?.canister as string,
    unauthenticatedAgent,
    {
      enabled: isConnected && !!unauthenticatedAgent && !!randomSelectedNFTId,
      nft_id: randomSelectedNFTId?.tokenId.id_bigint as bigint,
      nft_id_string: randomSelectedNFTId?.tokenId.id_string as string,
      placeholderData: {
        amount: 0,
        amount_e8s: 0n,
      },
    }
  );

  const balanceOGY = useFetchLedgerBalance(
    OGY_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "OGY",
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const insufficientOGYFunds =
    balanceOGY.isSuccess &&
    txFeeNFT.isSuccess &&
    balanceOGY.data.balance < txFeeNFT.data.amount * totalNFTSelected;

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

        <div className="flex justify-between items-center mt-8 mx-2">
          <div className="flex justify-start items-center text-content/60 text-sm rounded-lg">
            <div>Fee: </div>
            {txFeeNFT.isSuccess ? (
              <div className="flex items-center">
                <Logo name="ogy" className="mx-2 h-4 w-4" />
                <span>
                  <NumberToLocaleString
                    value={txFeeNFT.data.amount * totalNFTSelected}
                    decimals={3}
                  />{" "}
                  OGY
                </span>
              </div>
            ) : (
              <div>Fetching NFT fee...</div>
            )}
          </div>

          <div className="px-4 py-1 bg-surface-secondary text-content/60 text-xs rounded-md">
            <BalanceAvailable token="OGY" balance={balanceOGY.data?.balance} />
          </div>
        </div>

        {insufficientOGYFunds && (
          <DisclaimerInsufficientOGYFunds
            totalNFTSelected={totalNFTSelected}
            txFee={txFeeNFT.data.amount}
            balanceOGY={balanceOGY.data.balance}
            className="mt-8"
          />
        )}

        <BtnPrimary
          type="submit"
          disabled={
            !isValid ||
            totalNFTSelected === 0 ||
            !balanceOGY.isSuccess ||
            insufficientOGYFunds
          }
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
