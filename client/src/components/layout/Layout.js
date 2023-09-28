import React from 'react';
import Grid from './Grid';
import Footer from './Footer';
import { Box, Container } from '@chakra-ui/react';
import dynamic from 'next/dynamic';

const Layout = ({ children }) => {
    const Header = dynamic(() => import('@/components/ui/header/Header'), {
        ssr: false,
    });

    return (
        <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
            <Header />
            <Grid>{children}</Grid>
            <Footer />
        </Box>
    );
};

export default Layout;
