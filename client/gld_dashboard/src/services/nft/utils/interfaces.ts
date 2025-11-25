export type MetadataNFT = {
  name: string;
  description: string;
  image: string;
  attributes: Array<{
    trait_type: string;
    value: string | number;
  }>;
};

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
