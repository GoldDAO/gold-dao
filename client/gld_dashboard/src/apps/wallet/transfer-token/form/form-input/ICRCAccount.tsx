import { useEffect } from "react";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { useForm, useWatch } from "react-hook-form";
import clsx from "clsx";
import { useAtom } from "jotai";
import { isValidPrincipalOrICRCAccount } from "@shared/utils/validators/isValidPrincipalOrICRCAccount";
import { SendTokenStateAtom } from "@wallet/shared/atoms/TransferTokenAtom";
import InputCard from "../card";

const ICRCAccount = () => {
  const {
    register,
    control,
    setValue,
    reset,
    formState: { errors, isValid },
  } = useForm({
    mode: "onChange",
    shouldUnregister: true,
    shouldFocusError: false,
  });
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);
  const { receive_account } = sendState;

  const principal = useWatch({
    control,
    name: "principal",
    defaultValue: "",
  });

  useEffect(() => {
    if (principal && isValid) {
      setSendState((state) => ({
        ...state,
        receive_account: principal,
        is_principal_standard: true,
        is_valid_receive_address: true,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isValid, principal]);

  useEffect(() => {
    if (
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

  useEffect(() => {
    if (errors.principal) {
      setSendState((state) => ({
        ...state,
        error_message_receive_address: errors,
      }));
    }
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
    if (isValidPrincipalOrICRCAccount(value)) return true;
    return false;
  };

  return (
    <>
      <div className="text-copper text-sm font-semibold mb-2">
        Principal ID (or ICRC account)
      </div>
      <InputCard>
        <input
          id="principal"
          type="text"
          autoComplete="off"
          placeholder="Enter Principal or ICRC Account"
          className={clsx(
            "w-full outline-none focus:outline-none focus:ring-0 bg-surface-secondary",
            "placeholder:text-content/40"
          )}
          {...register("principal", {
            required: "Principal ID or ICRC Account is required",
            validate: {
              validatePrincicpal: (v) =>
                validatePrincipal(v) || "Invalid Principal ID or ICRC Account",
            },
          })}
        />
      </InputCard>
    </>
  );
};

export default ICRCAccount;
