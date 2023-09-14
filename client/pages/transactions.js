import Layout from '@/components/UI/layout/Layout';
import { Box } from '@mui/material';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';
import styled from 'styled-components';

const TransactionsTable = dynamic(() => import('@/components/UI/table/TransactionTable'), {
    ssr: false,
});

const Transactions = () => {
    return (
        <>
            <Head>
                {/* <title>{meta.title}</title>
            <meta property={`og:title`} content={meta.title} key="title" />
            <meta property={`og:description`} content={meta.description} key="title" /> */}
            </Head>
            <Layout>
                <PageTitle>Latest GLDT transactions</PageTitle>
                <TransactionsTable />
            </Layout>
        </>
    );
};

export default Transactions;

const PageTitle = styled(Box)`
    height: fit-content;
    font-size: 48px;
    font-weight: 300;
    position: relative;
    grid-column: span 12;
    width: 100%;
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
