import Layout from '@/components/layout/Layout';
import { useConnect } from '@connect2ic/react';
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
            </Head>
            <Layout>
                <SwapInterface />
            </Layout>
        </>
    );
}

export default Home;
