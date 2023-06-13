import { atom } from "jotai"

export const defaultAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}

export const goldNft1000GAgentAtom = atom(defaultAgent)

export const setGetgoldNft1000GAgentAtom = atom(
    (get) => get(goldNft1000GAgentAtom),
    (get, set, agent) => {
        set(goldNft1000GAgentAtom, agent)
    }
)


