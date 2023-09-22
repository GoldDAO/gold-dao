import { createClient } from '@connect2ic/core';
import { Connect2ICProvider } from '@connect2ic/react';
import { Provider as JotaiProvider, createStore } from 'jotai';
import { defaultProviders } from '@connect2ic/core/providers';
import { SafeHydrate } from '@/utils/SafeHydrate';
import '@connect2ic/core/style.css';
import { ChakraProvider } from '@chakra-ui/react';
import { gldNftCanisters, gldtLedgerCanister, gldtCoreCanister } from '@/services/agents/';

const myStore = createStore();

console.log('gldtCoreCanister', gldtCoreCanister);

const Providers = ({ children }) => {
    const whitelist = Object.values(gldNftCanisters).map((canister) => canister.canisterId);
    whitelist.push(gldtLedgerCanister.canisterId);
    whitelist.push(gldtCoreCanister.canisterId);
    const canisters = {
        ...gldNftCanisters,
        gldtLedgerCanister,
        gldtCoreCanister,
    };

    console.log('canisters', canisters);
    let client = createClient({
        canisters,
        providers: defaultProviders,
        globalProviderConfig: {
            host: 'https://icp0.io',
            dev: false,
            whitelist,
        },
    });

    if (!client) {
        return <></>;
    }

    return (
        <ChakraProvider>
            <Connect2ICProvider client={client}>
                <SafeHydrate>
                    <JotaiProvider store={myStore}>{children}</JotaiProvider>
                </SafeHydrate>
            </Connect2ICProvider>
        </ChakraProvider>
    );
};

export default Providers;
