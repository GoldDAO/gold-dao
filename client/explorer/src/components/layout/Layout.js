import React from 'react';
import { Box } from '@chakra-ui/react';
import Header from './Header';

const Layout = ({ children }) => {
    return (
        <Box sx={{ display: 'flex', flexDirection: 'column' }} p={['20px', '20px', '40px']}>
            <Header />
            {children}
        </Box>
    );
};

export default Layout;
