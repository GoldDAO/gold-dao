import { atom } from 'jotai'
import small from './../../public/images/gold/10g.png'
import medium from './../../public/images/gold/100g.png'
import big from './../../public/images/gold/1kg.png'

const nfts = [
    { name: 'asdf', image: small, id: 'asdf-1', weight: 10, unit: 'g' },
    { name: 'asdf', image: small, id: 'asdf-2', weight: 10, unit: 'g' },
    { name: 'asdf', image: small, id: 'asdf-3', weight: 10, unit: 'g' },
    { name: 'asdf', image: small, id: 'asdf-4', weight: 10, unit: 'g' },
    { name: 'asdf', image: small, id: 'asdf-5', weight: 10, unit: 'g' },
]

export const nftsAtom = atom([...nfts])

export const getNftsAtom = atom((get) => get(nftsAtom))

