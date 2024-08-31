export * from "./gldt_swap"

export interface SwapData {
  type: string;
  label: string;
  created_at: string;
  nft_id_string: string;
  send_value: number;
  receive_value: number;
  gldt_value: number;
  nft_value: number;
  status: { value: string; label: string };
  nft_id: string;
  index: string;
}
