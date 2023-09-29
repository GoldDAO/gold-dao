import Layout from '@/components/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

const MyAccount = () => {
    const Summary = dynamic(() => import('@/components/ui/manage/Summary'), {
        ssr: false,
    });
    const meta = {
        title: 'GLDT Swap App | Manage NFTs',
        description: 'GLDT Swap App Description',
    };
    return (
        <>
            <Head>
                <title>{'GLDT swap | my account'}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
            </Head>
            <Layout>
                <Summary />
            </Layout>
        </>
    );
};

export default MyAccount;
