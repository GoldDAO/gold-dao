import { atomWithReset } from "jotai/utils";

export const TransferStateAtom = atomWithReset<{
  is_open_transfer_dialog: boolean;
  transfer_tab: "send" | "receive";
}>({
  is_open_transfer_dialog: false,
  transfer_tab: "send",
});

export const SendStateAtom = atomWithReset<{
  is_step_send_form: boolean;
  is_step_send_confirm: boolean;
}>({
  is_step_send_form: true,
  is_step_send_confirm: false,
});
