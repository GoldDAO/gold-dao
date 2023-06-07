import { atom } from 'jotai'


export const userAtom = atom(null)

export const setGetUserAtom = atom(
    (get) => get(userAtom),
    (get, set, user) => {
        set(userAtom, user)
    }
)
