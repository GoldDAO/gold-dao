import { useEffect } from "react";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useForm, useWatch } from "react-hook-form";
import clsx from "clsx";
import { useAtom } from "jotai";
import { isValidPrincipalOrICRCAccount } from "@shared/utils/validators/isValidPrincipalOrICRCAccount";
import { isValidAccount } from "@shared/utils/validators/isValidAccount";
import { SendTokenStateAtom } from "@wallet/shared/atoms/TransferTokenAtom";
import InputCard from "../card";

const ICRCAccountOrAccountId = () => {
  const {
    register,
    control,
    setValue,
    reset,
    formState: { errors },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);
  const { receive_account } = sendState;

  const watchedPrincipal = useWatch({
    control,
    name: "principal",
    defaultValue: "",
  });

  // The send state follows the field: a valid account is the recipient, any
  // other text leaves no recipient and disables the Transfer button. The text
  // is checked here, because the form's isValid still holds the result for
  // the previous text when this effect runs.
  useEffect(() => {
    const isAccountId = isValidAccount(watchedPrincipal);
    const isValidRecipient =
      isAccountId || isValidPrincipalOrICRCAccount(watchedPrincipal);
    setSendState((state) => ({
      ...state,
      receive_account: isValidRecipient ? watchedPrincipal : "",
      is_principal_standard: isValidRecipient
        ? !isAccountId
        : state.is_principal_standard,
      is_valid_receive_address: isValidRecipient,
    }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [watchedPrincipal]);

  // Puts the recipient back into the field, e.g. on the way back from the
  // review step.
  useEffect(() => {
    if (isValidAccount(receive_account)) {
      setValue("principal", receive_account, {
        shouldValidate: true,
      });
    } else if (
      receive_account !== "" &&
      isValidPrincipalOrICRCAccount(receive_account)
    ) {
      const decoded = decodeIcrcAccount(receive_account);
      const encoded = encodeIcrcAccount({
        owner: decoded.owner,
        subaccount: decoded.subaccount ? decoded.subaccount : [],
      });
      setValue("principal", encoded, {
        shouldValidate: true,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [receive_account]);

  // The form shows this message below the inputs. It goes away once the
  // field is valid again.
  useEffect(() => {
    setSendState((state) => ({
      ...state,
      error_message_receive_address: errors.principal
        ? { principal: errors.principal }
        : {},
    }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [errors.principal]);

  useEffect(() => {
    return () => {
      reset();
      setSendState((state) => ({
        ...state,
        error_message_receive_address: {},
      }));
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const validatePrincipal = (value: string) => {
    if (isValidPrincipalOrICRCAccount(value) || isValidAccount(value))
      return true;
    return false;
  };

  return (
    <>
      <div className="text-copper text-sm font-semibold mb-2">
        Principal ID or Account ID (or ICRC Account)
      </div>
      <InputCard>
        <input
          id="principal"
          type="text"
          autoComplete="off"
          placeholder="Enter Principal ID or Account ID (or ICRC Account)"
          className={clsx(
            "w-full outline-none focus:outline-none focus:ring-0 bg-surface-secondary",
            "placeholder:text-content/40"
          )}
          {...register("principal", {
            required: "Principal ID or Account ID or ICRC Account is required",
            validate: {
              validatePrincipal: (v) =>
                validatePrincipal(v) ||
                "Invalid Principal ID or Account ID or ICRC Account",
            },
          })}
        />
      </InputCard>
    </>
  );
};

export default ICRCAccountOrAccountId;
