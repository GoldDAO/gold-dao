import { atom } from "jotai"

export const defaultAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}

export const goldNft10GAgentAtom = atom(defaultAgent)

export const setGetgoldNft10GAgentAtom = atom(
    (get) => get(goldNft10GAgentAtom),
    (get, set, agent) => {
        set(goldNft10GAgentAtom, agent)
    }
)


