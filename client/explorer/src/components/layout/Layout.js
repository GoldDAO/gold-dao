import React from 'react';
import Grid from '@ui/layout/Grid';
import { Box } from '@chakra-ui/react';
import Header from './Header';

const Layout = ({ children }) => {
    return (
        <Box sx={{ display: 'flex', flexDirection: 'column' }} p="40px">
            <Header />
            <Grid>{children}</Grid>
        </Box>
    );
};

export default Layout;
