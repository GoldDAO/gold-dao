import { Inter } from 'next/font/google';
import '@/css/global.css';
import '@connect2ic/core/style.css';
import dynamic from 'next/dynamic';

export const inter = Inter({
    subsets: ['latin'],
    weight: ['200', '300', '400', '500'],
    fallback: ['system-ui', 'arial', 'sans-serif'],
});

const Providers = dynamic(() => import('@/components/Providers'), {
    ssr: false,
});

export default function MyApp({ Component, pageProps }) {
    return (
        <div className={inter.className}>
            <Providers>
                <Component {...pageProps} />
            </Providers>
        </div>
    );
}
