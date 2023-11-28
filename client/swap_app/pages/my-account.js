import { Layout } from '@/components/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';
import React from 'react';
import Metas from '@ui/layout/Metas';

const MyAccount = () => {
    const Summary = dynamic(() => import('@/components/ui/manage/Summary'), {
        ssr: false,
    });
    const meta = {
        title: 'GLDT Swap App | Manage NFTs',
        description: 'GLDT Swap App Description',
    };
    return (
        <>
            <Metas meta={meta} />
            <Layout>
                <Summary />
            </Layout>
        </>
    );
};

export default MyAccount;
