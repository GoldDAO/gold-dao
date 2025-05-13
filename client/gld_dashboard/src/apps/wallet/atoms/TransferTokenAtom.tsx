import { atomWithReset } from "jotai/utils";

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
  principal: string;
  subaccount: string;
  receive_account: string;
  is_icrc_account: boolean;
  is_principal_standard: boolean;
  is_step_send_form: boolean;
  is_step_send_confirm: boolean;
}>({
  amount: 0n,
  amount_input: "",
  principal: "",
  subaccount: "",
  receive_account: "",
  is_icrc_account: false,
  is_principal_standard: true,
  is_step_send_form: true,
  is_step_send_confirm: false,
});
