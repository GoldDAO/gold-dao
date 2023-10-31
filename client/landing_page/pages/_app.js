import { ChakraProvider } from '@chakra-ui/react';
import { Inter } from 'next/font/google';
import { useEffect } from 'react';
import '../src/css/global.css';
const inter = Inter({ subsets: ['latin'] });
import { Connect2ICProvider } from '@connect2ic/react';

export default function MyApp({ Component, pageProps }) {
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

    return (
        <div className={inter.className}>
            <ChakraProvider>
                <Connect2ICProvider client={client}>
                    <Component {...pageProps} />
                </Connect2ICProvider>
            </ChakraProvider>
        </div>
    );
}
