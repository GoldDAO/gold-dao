import Layout from '@/components/UI/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

const MyNftsTable = dynamic(() => import('@/components/UI/table/NftsTable'), {
    ssr: false,
});

const MyNfts = () => {
    return (
        <>
            <Head>
                {/* <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" /> */}
            </Head>
            <Layout>
                <MyNftsTable />
            </Layout>
        </>
    );
};

export default MyNfts;