import AppStatus from '@/components/UI/feedback/AppStatus';
import Layout from '@/components/UI/layout/Layout';
import TextSection from '@/components/UI/sections/TextSection;';
import { SectionTitle } from '@/components/UI/styled/common';
import { onSaleNftAtom } from '@/states/onSalesNfts';
import { userAtom } from '@/states/user';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';
import { useEffect } from 'react';

const OngoingSwapsTable = dynamic(() => import('@/components/UI/table/OngoingSwapsTable'), {
    ssr: false,
});

const OngoingSwaps = () => {
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
                        <OngoingSwapsTable />
                    </>
                ) : (
                    <AppStatus />
                )}
            </Layout>
        </>
    );
};

export default OngoingSwaps;
