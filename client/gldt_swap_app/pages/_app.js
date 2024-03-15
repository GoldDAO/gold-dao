import { Inter } from 'next/font/google';
import '@/css/global.css';
import '@connect2ic/core/style.css';
import dynamic from 'next/dynamic';
import Ruban from '@ui/header/Ruban';
import { useEffect } from 'react';

const inter = Inter({ subsets: ['latin'] });

const Providers = dynamic(() => import('@/components/c2ic/Providers'), {
    ssr: false,
});

export default function MyApp({ Component, pageProps }) {
    useEffect(() => {
        console.log(
            `GLDT Swap ${process.env.FRONTEND_VERSION} interface with swap engine ${process.env.CORE_VERSION}.`,
        );
    }, []);
    return (
        <div className={inter.className}>
            {process.env.DFX_NETWORK !== 'ic' && <Ruban />}
            <Providers>
                <Component {...pageProps} />
            </Providers>
        </div>
    );
}
