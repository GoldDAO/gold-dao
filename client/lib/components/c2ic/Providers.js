import Layout from "../UI/layout/Layout";
import { createClient } from "@connect2ic/core";
import { Connect2ICProvider } from "@connect2ic/react";
import { Provider as JotaiProvider, createStore } from 'jotai'
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { defaultProviders } from "@connect2ic/core/providers";
import { SafeHydrate } from "../../utils/SafeHydrate";
import dynamic from 'next/dynamic';
import "@connect2ic/core/style.css";
import * as NFT_1G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_1g_NFT'
import * as NFT_10G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_10g_NFT'
import * as NFT_100G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_100g_NFT'
import * as NFT_1000G_CANISTER from '../../../src/agents/declarations/NFTs/GOLD_1000g_NFT'

const myStore = createStore()

const Providers = ({ children }) => {
    let client = createClient({
        canisters: {
            'NFT_1G_CANISTER': NFT_1G_CANISTER,
            'NFT_10G_CANISTER': NFT_10G_CANISTER,
            'NFT_100G_CANISTER': NFT_100G_CANISTER,
            'NFT_1000G_CANISTER': NFT_1000G_CANISTER,
        },
        providers: defaultProviders,
        globalProviderConfig: {
            host: "https://ic0.app",
            dev: false,
            whitelist: [
                NFT_1G_CANISTER.canisterId,
                NFT_10G_CANISTER.canisterId,
                NFT_100G_CANISTER.canisterId,
                NFT_1000G_CANISTER.canisterId
            ],
        },
    });

    if (!client) {
        return <></>;
    }

    return (
        <JotaiProvider store={myStore}>
            <Connect2ICProvider client={client}>
                <SafeHydrate>
                    {children}
                </SafeHydrate>
            </Connect2ICProvider>
        </JotaiProvider>

    );
};


export default Providers