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
  receive_address: string | null;
  fee: bigint | null;
  decimals: number | null;
  is_step_send_form: boolean;
  is_step_send_confirm: boolean;
}>({
  amount: 0n,
  amount_input: "",
  receive_address: null,
  fee: null,
  decimals: null,
  is_step_send_form: true,
  is_step_send_confirm: false,
});
