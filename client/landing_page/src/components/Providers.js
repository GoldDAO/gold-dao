import { createClient } from '@connect2ic/core';
import { Connect2ICProvider } from '@connect2ic/react';
import { SafeHydrate } from '@utils/SafeHydrate';
import '@connect2ic/core/style.css';
import { ChakraProvider, extendTheme } from '@chakra-ui/react';
import { gldtLedgerCanister } from './../../../utils/agents';
import { customTheme } from '@ui/theme';

const Providers = ({ children }) => {
    const whitelist = [gldtLedgerCanister.canisterId];
    const canisters = {
        gldtLedgerCanister,
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
