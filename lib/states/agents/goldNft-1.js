import { atom } from "jotai"

export const defaultAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}

export const goldNft1GAgentAtom = atom(defaultAgent)

export const setGetgoldNft1GAgentAtom = atom(
    (get) => get(goldNft1GAgentAtom),
    (get, set, agent) => {
        set(goldNft1GAgentAtom, agent)
    }
)


