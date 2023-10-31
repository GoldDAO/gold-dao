import { Inter } from 'next/font/google';
import { useEffect } from 'react';
import '../src/css/global.css';
import dynamic from 'next/dynamic';
const inter = Inter({ subsets: ['latin'] });

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
