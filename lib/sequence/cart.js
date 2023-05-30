import { atom } from 'jotai'

const product = {
    id: '',
    weight: 0,
}

export const cartAtom = atom([])

const readWriteAtom = atom(
    (get) => get(cartAtom),
    (get, set, newCart) => {
        set(cartAtom, newCart)
    }
)


