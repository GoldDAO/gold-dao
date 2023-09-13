import AppStatus from '@/components/UI/feedback/AppStatus';
import Layout from '@/components/UI/layout/Layout';
import { userAtom } from '@/states/user';
import { useAtom } from 'jotai';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

const MyNftsTable = dynamic(() => import('@/components/UI/table/NftsTable'), {
    ssr: false,
});

const MyNfts = () => {
    const [user] = useAtom(userAtom);

    return (
        <>
            <Head>
                {/* <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" /> */}
            </Head>
            <Layout>
                {user.isConnected ? (
                    <>
                        <MyNftsTable selectable={false} hasControls={true} />
                    </>
                ) : (
                    <AppStatus />
                )}
            </Layout>
        </>
    );
};

export default MyNfts;
