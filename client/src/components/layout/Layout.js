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
        <Container sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
            <Container>
                <Grid>
                    <Header />
                    <Box>
                        <Grid>{children}</Grid>
                    </Box>
                </Grid>
            </Container>
            <Footer />
        </Container>
    );
};

export default Layout;
