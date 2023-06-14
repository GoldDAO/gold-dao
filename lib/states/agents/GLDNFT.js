import { atom } from "jotai"

const emptyAgent = {
    agent: {
        identity: undefined,
        host: undefined,
    },
    identity: undefined,
    isAuthed: false,
}

export const defaultAgent = {
    NFT_1g: {
        emptyAgent
    },
    NFT_10g: {
        emptyAgent
    },
    NFT_100g: {
        emptyAgent
    },
    NFT_1000g: {
        emptyAgent
    },

}

export const NftGAgentAtom = atom(defaultAgent)

// export const setGetNftGAgentAtom = atom(
//     (get) => get(NftGAgentAtom),
//     (get, set, agent) => {
//         set(NftGAgentAtom, agent)
//     }
// )



export const goldNft1GAgentAtom = atom(defaultAgent)

export const setGetgoldNft1GAgentAtom = atom(
    (get) => get(goldNft1GAgentAtom),
    (get, set, agent) => {
        set(goldNft1GAgentAtom, agent)
    }
)


