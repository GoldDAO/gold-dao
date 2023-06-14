import React, { useEffect } from 'react';
import { createClient } from "@connect2ic/core";
import { Connect2ICProvider } from "@connect2ic/react";
import { defaultProviders } from "@connect2ic/core/providers";
import "@connect2ic/core/style.css";
import * as NFT_1G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_1g_NFT'
import * as NFT_10G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_10g_NFT'
import * as NFT_100G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_100g_NFT'
import * as NFT_1000G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_1000g_NFT'

const C2ICProvider = ({ children }) => {


    let client = createClient({
        canisters: {
            // NFT_1G_CANISTER: NFT_1G_CANISTER,
            // NFT_10G_CANISTER: NFT_10G_CANISTER,
            // NFT_100G_CANISTER: NFT_100G_CANISTER,
            // NFT_1000G_CANISTER: NFT_1000G_CANISTER,
        },
        providers: defaultProviders,
        globalProviderConfig: {
            host: "https://ic0.app",
            dev: false,
        },
        whitelist: [
            // NFT_1G_CANISTER.canisterId,
            // NFT_10G_CANISTER.canisterId,
            // NFT_100G_CANISTER.canisterId,
            // NFT_1000G_CANISTER.canisterId
        ],
    });

    useEffect(() => {
        console.log('client', client)
    }, [client])

    return (
        <Connect2ICProvider client={client} >
            {children}
        </Connect2ICProvider >
    );
};

export default C2ICProvider;