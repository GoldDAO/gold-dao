import Question from '@/components/FAQ/Question';
import Layout from '@/components/layout/Layout';
import { Accordion, Box, Heading } from '@chakra-ui/react';
import React from 'react';

const content = {
    q: 'What is a GLDT? ',
    r: 'GLDT stands for “Gold token” and is a fungible token which is backed by GLD NFT. GLD NFTs are NFTs that represent ownership of physical gold bars (learn more here). 1 GLDT is exactly 0.01g of gold and GLD NFTs can be swapped at a ratio of 1g for 100 GLDT.',
};

const FAQ = () => {
    const meta = {
        title: 'GLDT Swap FAQ',
        description: 'GLDT Swap FAQ Description',
    };
    return (
        <Layout meta={meta}>
            <Box m="0 auto" px={['20px', '40px', '60px', '150px', '280px']}>
                <Heading as="h1" fontSize={'96px'} my="100px">
                    Frequently Asked Questions
                </Heading>
                <Accordion allowToggle>
                    {Array.from({ length: 20 }).map((e, i) => (
                        <Question key={i} q={content.q} r={content.r} />
                    ))}
                </Accordion>
            </Box>
        </Layout>
    );
};

export default FAQ;
