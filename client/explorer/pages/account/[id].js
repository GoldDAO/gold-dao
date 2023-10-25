import React from 'react';
import Layout from '../../src/components/Layout';
import { Box, Heading } from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useState } from 'react';
import dynamic from 'next/dynamic';

const AccountPage = () => {
    const { query } = useRouter();

    const AccountContent = dynamic(() => import('../../src/components/account/AccountContent'), {
        ssr: false,
    });

    return (
        <Layout>
            <AccountContent id={query.id} subAccount={query.subaccount} />
        </Layout>
    );
};

export default AccountPage;
