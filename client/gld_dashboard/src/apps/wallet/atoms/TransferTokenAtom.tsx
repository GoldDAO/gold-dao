import { atomWithReset } from "jotai/utils";
import { DeepMap, FieldError, FieldErrors, FieldValues } from "react-hook-form";

export const TransferTokenStateAtom = atomWithReset<{
  is_open_transfer_dialog: boolean;
  transfer_tab: "send" | "receive";
}>({
  is_open_transfer_dialog: false,
  transfer_tab: "send",
});

export const SendTokenStateAtom = atomWithReset<{
  amount: bigint;
  amount_input: string;
  is_principal_standard: boolean;
  receive_account: string;
  is_valid_receive_address: boolean;
  error_message_receive_address: FieldErrors<FieldValues>;
  is_use_icrc_account: boolean;
  is_step_send_form: boolean;
  is_step_send_confirm: boolean;
}>({
  amount: 0n,
  amount_input: "",
  receive_account: "",
  is_valid_receive_address: false,
  is_principal_standard: true,
  error_message_receive_address: {} as DeepMap<FieldValues, FieldError>,
  is_use_icrc_account: true,
  is_step_send_form: true,
  is_step_send_confirm: false,
});
