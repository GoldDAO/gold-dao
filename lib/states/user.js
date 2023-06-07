import { atom } from 'jotai'


export const userAtom = atom({
    principal: undefined,
    connect: undefined,
    disconnect: undefined,
    status: undefined,
    isInitializing: undefined,
    isIdle: undefined,
    isConnecting: undefined,
    isConnected: false,
    isDisconnecting: undefined,
    activeProvider: undefined
})

export const setGetUserAtom = atom(
    (get) => get(userAtom),
    (get, set, user) => {
        set(userAtom, user)
    }
)
