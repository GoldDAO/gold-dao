import { atom } from "jotai"

export const defaultAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}
export const agentAtom = atom(defaultAgent)

export const setGetAgentAtom = atom(
    (get) => get(agentAtom),
    (get, set, agent) => {
        set(agentAtom, agent)
    }
)


