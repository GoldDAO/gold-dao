import Layout from '@/components/UI/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

const MyNfts = dynamic(() => import('@/components/UI/table/MyNfts'), {
    ssr: false,
});

const ManageNft = () => {
    return (
        <>
            <Head>
                {/* <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" /> */}
            </Head>
            <Layout>
                <MyNfts />
            </Layout>
        </>
    );
};

export default ManageNft;