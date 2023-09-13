import Layout from '@/components/UI/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';

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
                <TransactionsTable />
            </Layout>
        </>
    );
};

export default Transactions;
