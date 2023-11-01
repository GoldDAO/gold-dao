import { Layout } from '@/components/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';

function Home({}) {
    const meta = {
        title: 'GLDT Swap App',
        description: 'GLDT Swap App Description',
    };
    const SwapInterface = dynamic(() => import('@/components/ui/swap/Swap'), {
        ssr: false,
    });

    return (
        <>
            <Head>
                <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
                <link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png" />
                <meta name="msapplication-TileColor" content="#da532c" />
                <meta name="theme-color" content="#ffffff" />
            </Head>
            <Layout>
                <SwapInterface />
            </Layout>
        </>
    );
}

export default Home;
