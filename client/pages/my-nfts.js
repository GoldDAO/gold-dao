import AppStatus from '@/components/UI/feedback/AppStatus';
import Layout from '@/components/UI/layout/Layout';
import { userAtom } from '@/states/user';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';
import styled from 'styled-components';

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
                <PageTitle>My Nfts</PageTitle>
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

const PageTitle = styled(Box)`
    height: fit-content;
    font-size: 48px;
    font-weight: 300;
    position: relative;
    grid-column: span 12;
    width: 100%;
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    h1,
    h2 {
        position: relative;
        z-index: 2;
        grid-column: 2/7;
        line-height: 1.2em;
        span {
            position: relative;
            left: -10px;
        }
    }
`;
