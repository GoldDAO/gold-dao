import { Inter } from 'next/font/google';
import '../src/css/global.css';
import dynamic from 'next/dynamic';

export const inter = Inter({
    subsets: ['latin'],
    weight: ['300', '400', '500'],
    fallback: ['system-ui', 'arial', 'sans-serif'],
});

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
