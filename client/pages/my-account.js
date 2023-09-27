import Layout from '@/components/layout/Layout';
import dynamic from 'next/dynamic';
import React from 'react';

const MyAccount = () => {
    const Summary = dynamic(() => import('@/components/ui/manage/Summary'), {
        ssr: false,
    });

    return (
        <Layout>
            <Summary />
        </Layout>
    );
};

export default MyAccount;
