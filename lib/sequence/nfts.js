import { atom } from 'jotai'
import small from './../../public/images/gold/10g.png'
import medium from './../../public/images/gold/100g.png'
import big from './../../public/images/gold/1kg.png'

const nfts = [
    { name: 'asdf', image: small, id: 'asdf-1', weight: 10, checked: false },
    { name: 'asdf', image: small, id: 'asdf-2', weight: 10, checked: false },
    { name: 'asdf', image: small, id: 'asdf-3', weight: 10, checked: false },
    { name: 'asdf', image: small, id: 'asdf-4', weight: 10, checked: false },
    { name: 'asdf', image: small, id: 'asdf-5', weight: 10, checked: false },
]

export const nftsAtom = atom(nfts)

const readWriteAtom = atom(
    (get) => get(nftsAtom),
    (get, set, newNfts) => {
        set(nftsAtom, newNfts)
    }
)


