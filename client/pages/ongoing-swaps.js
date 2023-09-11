import Layout from '@/components/UI/layout/Layout';
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
    const [onSale] = useAtom(onSaleNftAtom);
    console.log('onSale', onSale);
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
                        <OngoingSwapsTable on_sale={onSale} />
                    </>
                ) : (
                    <Box>Please connect your wallet</Box>
                )}
            </Layout>
        </>
    );
};

export default OngoingSwaps;
