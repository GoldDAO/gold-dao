import { Box } from '@mui/system';
import React from 'react';
import Header from './Header';

const Layout = ({children}) => {
    return (
        <Box>
            <Header />
            <Box>
                {children}
            </Box>
        </Box>
    );
};

export default Layout;