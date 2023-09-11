import { atom } from 'jotai';

export const onSaleNftAtom = atom([]);

export const updateOnsaleNft = (nft, onSaleNftAtom) => [...onSaleNftAtom, nft];

export const updateOnsaleNftAtom = atom(null, (get, set, nft) => {
    if (nft !== undefined && isDuplicate(nft, get(onSaleNftAtom)) === false) {
        set(onSaleNftAtom, updateOnsaleNft(nft, get(onSaleNftAtom)));
    }
});

export const emptyOnGoing = () => [];

export const emptyOnGoingAtom = atom(null, (_get, set) => {
    set(onSaleNftAtom, emptyOnGoing());
});

const isDuplicate = (target, array) => {
    for (const obj of array) {
        let isEqual = true;
        for (const key in obj) {
            if (obj[key] !== target[key]) {
                isEqual = false;
                break;
            }
        }
        if (isEqual) {
            return true;
        }
    }
    return false;
};
