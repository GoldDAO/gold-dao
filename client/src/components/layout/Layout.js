import React from 'react';
import Header from './Header';
import Grid from './Grid';
import Footer from './Footer';
import { Box, Container } from '@chakra-ui/react';

const Layout = ({ children }) => {
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
