import { Inter } from 'next/font/google';
import { useEffect } from 'react';
import '../src/css/global.css';
import dynamic from 'next/dynamic';
import { ChakraProvider, extendTheme } from '@chakra-ui/react';
import { customTheme } from '@ui/theme';

const inter = Inter({ subsets: ['latin'], weight: 'variable' });

export default function MyApp({ Component, pageProps }) {
    const Providers = dynamic(() => import('@/components/Providers'), {
        ssr: false,
    });

    return (
        <div className={inter.className}>
            <Providers>
                <Component {...pageProps} />
            </Providers>
        </div>
    );
}
