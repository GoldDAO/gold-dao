import { atom } from 'jotai'

export const onSaleNftAtom = atom([])

export const updateOnsaleNft = (nft, onSaleNftAtom) => [...onSaleNftAtom, nft]

export const updateOnsaleNftAtom = atom(
    null,
    (get, set, nft) => {
        set(onSaleNftAtom, updateOnsaleNft(nft, get(onSaleNftAtom)));
    }
)