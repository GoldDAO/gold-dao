import { createClient } from '@connect2ic/core';
import { Connect2ICProvider } from '@connect2ic/react';
import { SafeHydrate } from './SafeHydrate';
import '@connect2ic/core/style.css';
import { ChakraProvider } from '@chakra-ui/react';
import { gldNftCanisters, gldtLedgerCanister } from './../../../utils/agents';

const Providers = ({ children }) => {
    const whitelist = [gldtLedgerCanister.canisterId];
    const canisters = {
        gldtLedgerCanister,
    };

    console.log('canisters', canisters);

    let client = createClient({
        canisters,
        providers: [],
        globalProviderConfig: {
            host: 'https://icp0.io',
            dev: false,
            whitelist,
        },
    });

    return (
        <ChakraProvider>
            <Connect2ICProvider client={client}>
                <SafeHydrate>{children}</SafeHydrate>
            </Connect2ICProvider>
        </ChakraProvider>
    );
};

export default Providers;
