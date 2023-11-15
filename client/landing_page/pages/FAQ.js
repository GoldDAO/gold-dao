import Layout from '@/components/layout/Layout';
import { Accordion, Box, GridItem, Heading } from '@chakra-ui/react';
import React from 'react';
import FaqSection from '@/components/FAQ/FaqSection';

const FAQ = () => {
    const meta = {
        title: 'GLDT - FAQ',
        description: 'Frequently Asked Question about GLDT',
    };

    return (
        <Layout meta={meta}>
            <FaqSection full={true} />
        </Layout>
    );
};

export default FAQ;
