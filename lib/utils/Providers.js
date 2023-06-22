import Layout from "../components/UI/layout/Layout";
import { createClient } from "@connect2ic/core";
import { Connect2ICProvider } from "@connect2ic/react";
import { Provider as JotaiProvider, createStore } from 'jotai'
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { defaultProviders } from "@connect2ic/core/providers";
import { SafeHydrate } from "./SafeHydrate";
import dynamic from 'next/dynamic';

const myStore = createStore()

// const Root = () => {
//     console.log("Rendering Root");
//     return (
//         <Layout>
//         </Layout>
//     );
// };

const Providers = ({ children }) => {
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

    if (!client) {
        return <></>;
    }

    return (
        <SafeHydrate>
            <Connect2ICProvider client={client}>
                <JotaiProvider store={myStore}>
                    {children}
                </JotaiProvider>
            </Connect2ICProvider>
        </SafeHydrate>

    );
};


export default Providers