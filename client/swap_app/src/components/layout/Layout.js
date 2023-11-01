import React from 'react';
import Grid from '@ui/layout/Grid';
import Footer from './Footer';
import { Box } from '@chakra-ui/react';
import dynamic from 'next/dynamic';

export const Layout = ({ children }) => {
    const Header = dynamic(() => import('@/components/layout/Header'), {
        ssr: false,
    });
    return (
        <Box sx={{ display: 'flex', flexDirection: 'column' }}>
            <Header />
            <Grid>{children}</Grid>
            <Footer />
        </Box>
    );
};
