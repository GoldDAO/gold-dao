import React from 'react';
import Layout from '@/components/Layout';
import { Box, Heading } from '@chakra-ui/react';

const Transparency = () => {
    return (
        <Layout>
            <Box my="100px">
                <Heading as="h1" fontWeight={300}>
                    {' '}
                    Transparency
                </Heading>
            </Box>
        </Layout>
    );
};

export default Transparency;
