import { atom } from "jotai"

export const defaultAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}

export const goldNft100GAgentAtom = atom(defaultAgent)

export const setGetgoldNft100GAgentAtom = atom(
    (get) => get(goldNft100GAgentAtom),
    (get, set, agent) => {
        set(goldNft100GAgentAtom, agent)
    }
)


