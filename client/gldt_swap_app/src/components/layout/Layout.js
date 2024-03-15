import React from 'react';
import GridSystem from '@ui/layout/GridSystem';
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
            <GridSystem>{children}</GridSystem>
            <Footer />
        </Box>
    );
};
