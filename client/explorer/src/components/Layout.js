import React from 'react';
import Grid from './Grid';
import { Box, Container } from '@chakra-ui/react';
import Header from './header/Header';

const Layout = ({ children }) => {
    return (
        <Box sx={{ display: 'flex', flexDirection: 'column' }} p="40px">
            <Header />
            <Grid>{children}</Grid>
        </Box>
    );
};

export default Layout;
