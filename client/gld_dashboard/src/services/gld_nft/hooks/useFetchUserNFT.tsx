import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent, ActorSubclass } from "@dfinity/agent";

import { idlFactory } from "../idlFactory";
import { IdNFT, CollectionNameNFT } from "../utils/interfaces";

export const bigintTo32ByteArray = (value: bigint) => {
  const byteArray = new Uint8Array(32);
  for (let i = byteArray.length - 1; i >= 0; i--) {
    byteArray[i] = Number(value & 0xffn);
    value >>= 8n;
  }
  return byteArray.reverse();
};

const unlisted_tokens_of = async (
  actor: ActorSubclass,
  options: { owner: string; subaccount?: string[] }
) => {
  const { owner, subaccount = [] } = options;
  const result = (await actor.unlisted_tokens_of(
    {
      owner: Principal.fromText(owner),
      subaccount,
    },
    [],
    []
  )) as Array<bigint>;
  return result;
};

const get_nat_as_token_id_origyn = async (
  actor: ActorSubclass,
  options: { tokenId: bigint }
) => {
  const { tokenId } = options;
  const result = (await actor.get_nat_as_token_id_origyn(tokenId)) as string;
  return result;
};

const useFetchUserNft = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<IdNFT[]>, "queryKey" | "queryFn"> & {
    owner: string;
    subaccount?: string[];
    collectionName: CollectionNameNFT;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    owner,
    subaccount,
    collectionName,
  } = options;

  return useQuery({
    queryKey: [`FETCH_${collectionName}_NFT`, owner, subaccount],
    queryFn: async () => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const unlisted_tokens_of_result = await unlisted_tokens_of(actor, {
          owner,
          subaccount,
        });

        const result = await Promise.all(
          unlisted_tokens_of_result.map(
            async (tokenId: bigint): Promise<IdNFT> => {
              const result = (await get_nat_as_token_id_origyn(actor, {
                tokenId,
              })) as string;

              return {
                id_string: result,
                id_bigint: tokenId,
                id_byte_array: bigintTo32ByteArray(tokenId),
              };
            }
          )
        );
        return result ?? [];
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch ${collectionName} NFTs error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
  });
};

export default useFetchUserNft;
