import Layout from '@/components/UI/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

const NftBreakdownTable = dynamic(() => import('@/components/UI/table/NftBreakdownTable'), {
    ssr: false,
});

const NftBreakdown = () => {
    return (
        <>
            <Head>
                {/* <title>{meta.title}</title>
            <meta property={`og:title`} content={meta.title} key="title" />
            <meta property={`og:description`} content={meta.description} key="title" /> */}
            </Head>
            <Layout>
                <NftBreakdownTable />
            </Layout >
        </>
    );
};

export default NftBreakdown;