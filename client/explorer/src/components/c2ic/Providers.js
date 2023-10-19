import { createClient } from '@connect2ic/core';
import { Connect2ICProvider } from '@connect2ic/react';
import { InfinityWallet, NFID, defaultProviders } from '@connect2ic/core/providers';
import { SafeHydrate } from '@/utils/SafeHydrate';
import '@connect2ic/core/style.css';
import { ChakraProvider, extendBaseTheme, extendTheme } from '@chakra-ui/react';
import { gldNftCanisters, gldtLedgerCanister, gldtCoreCanister } from '@/services/agents/';
import { customTheme } from '@ui/theme';

const Providers = ({ children }) => {
    const whitelist = Object.values(gldNftCanisters).map((canister) => canister.canisterId);
    whitelist.push(gldtLedgerCanister.canisterId);
    whitelist.push(gldtCoreCanister.canisterId);
    const canisters = {
        ...gldNftCanisters,
        gldtLedgerCanister,
        gldtCoreCanister,
    };

    let client = createClient({
        canisters,
        providers: [],
        globalProviderConfig: {
            host: 'https://icp0.io',
            dev: false,
            whitelist,
        },
    });

    const theme = extendTheme({
        ...customTheme,
    });

    return (
        <ChakraProvider theme={theme}>
            <Connect2ICProvider client={client}>
                <SafeHydrate>{children}</SafeHydrate>
            </Connect2ICProvider>
        </ChakraProvider>
    );
};

export default Providers;
