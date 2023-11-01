import React from 'react';
import Layout from '../../src/components/layout/Layout';
import { useRouter } from 'next/router';
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
