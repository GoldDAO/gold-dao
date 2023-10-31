import React from 'react';
import Layout from '../../src/components/Layout';
import { Box, Heading } from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useState } from 'react';
import dynamic from 'next/dynamic';

const TransactionPage = () => {
    const { query } = useRouter();

    const TransactionContent = dynamic(
        () => import('../../src/components/transaction/TransactionContent'),
        {
            ssr: false,
        },
    );

    return (
        <Layout>
            <TransactionContent id={query.id} />
        </Layout>
    );
};

export default TransactionPage;
