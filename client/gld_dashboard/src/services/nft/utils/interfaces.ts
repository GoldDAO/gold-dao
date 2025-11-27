export type MetadataNFT = {
  name?: string;
  description?: string;
  img_preview?: string;
  weight?: string;
  value?: number;
  fineness?: string;
  dimensions?: string;
  hardness?: string;
  manufacturer?: string;
  serial_number?: number;
  img_front?: string;
  img_back?: string;
};

export type BlockTx = {
  tx_id: bigint;
  nft_id: bigint;
  created_at: string | null;
  from: string | null;
  to: string | null;
  type: string | null;
} & Partial<MetadataNFT>;

export type NFT = MetadataNFT & {
  id: bigint;
};

export type CollectionNameNFT = "1G" | "10G" | "100G" | "1KG";

export type NFTCollection = {
  canisterId: string;
  canisterIdIndexer: string;
  grams: number;
  name: CollectionNameNFT;
};
