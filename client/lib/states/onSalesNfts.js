import { atom } from 'jotai'

export const onSaleNft10Atom = atom([])

export const updateOnsaleNft = (nft, onSaleNft10Atom) => [...onSaleNft10Atom, nft]

export const updateOnsaleNftAtom = atom(
    null,
    (get, set, nft) => {
        set(onSaleNft10Atom, updateOnsaleNft(nft, get(onSaleNft10Atom)));
    }
)
