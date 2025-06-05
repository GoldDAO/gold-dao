import { useEffect } from "react";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";
import { useForm, useWatch } from "react-hook-form";
import clsx from "clsx";
import { Buffer } from "buffer";
import { useAtom } from "jotai";
import { isValidPrincipalAndSubaccount } from "@shared/utils/validators/isValidPrincipalAndSubaccount";
import { SendTokenStateAtom } from "@wallet/shared/atoms/TransferTokenAtom";
import { isValidPrincipalOrICRCAccount } from "@shared/utils/validators/isValidPrincipalOrICRCAccount";
import { isValidPrincipal } from "@shared/utils/validators/isValidPrincipal";
import InputCard from "../card";

const PrincipalAndSubaccount = () => {
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

  const watchedPrincipal = useWatch({
    control,
    name: "principal",
    defaultValue: "",
  });

  const watchedSubaccount = useWatch({
    control,
    name: "subaccount",
    defaultValue: "",
  });

  useEffect(() => {
    if (watchedPrincipal && isValidPrincipal(watchedPrincipal)) {
      const encoded = encodeIcrcAccount({
        owner: Principal.fromText(watchedPrincipal),
        subaccount: watchedSubaccount
          ? Buffer.from(watchedSubaccount, "hex")
          : [],
      });
      setSendState((state) => ({
        ...state,
        receive_account: encoded,
        is_principal_standard: true,
        is_valid_receive_address: true,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isValid, watchedPrincipal, watchedSubaccount]);

  useEffect(() => {
    if (
      receive_account !== "" &&
      isValidPrincipalOrICRCAccount(receive_account)
    ) {
      const decoded = decodeIcrcAccount(receive_account);
      setValue("principal", decoded.owner.toText(), {
        shouldValidate: true,
      });
      setValue(
        "subaccount",
        decoded.subaccount
          ? Buffer.from(decoded.subaccount).toString("hex")
          : "",
        {
          shouldValidate: true,
        }
      );
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
    if (isValidPrincipalAndSubaccount(value, watchedSubaccount)) return true;
    return false;
  };

  return (
    <div>
      <div className="flex flex-col md:flex-row md:items-end gap-4">
        <div className="flex-1">
          <div className="text-primary text-sm mb-2">Principal</div>
          <InputCard>
            <input
              id="principal"
              type="text"
              autoComplete="off"
              placeholder="Enter a Principal"
              className={clsx(
                "w-full outline-none focus:outline-none focus:ring-0 bg-surface-secondary",
                "placeholder:text-content/40"
              )}
              {...register("principal", {
                required: "Principal ID is required",
                validate: {
                  validatePrincipal: (v) =>
                    validatePrincipal(v) ||
                    "Invalid Principal ID or Subaccount",
                },
              })}
            />
          </InputCard>
        </div>

        <div className="flex-1">
          <div className="text-primary text-sm mb-2">Subaccount</div>
          <InputCard>
            <input
              id="subaccount"
              type="text"
              autoComplete="off"
              placeholder="Enter a Subaccount"
              className={clsx(
                "w-full outline-none focus:outline-none focus:ring-0 bg-surface-secondary",
                "placeholder:text-content/40"
              )}
              {...register("subaccount", {})}
            />
          </InputCard>
        </div>
      </div>
    </div>
  );
};

export default PrincipalAndSubaccount;
