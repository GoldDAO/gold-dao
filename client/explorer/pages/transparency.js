import React from 'react';
import Layout from '@/components/Layout';
import { Box, Heading } from '@chakra-ui/react';
import dynamic from 'next/dynamic';

const Transparency = () => {
    const TransparencyContent = dynamic(
        () => import('@/components/transparency/TransparencyContent'),
        {
            ssr: false,
        },
    );

    return (
        <Layout>
            <TransparencyContent />
        </Layout>
    );
};

export default Transparency;
