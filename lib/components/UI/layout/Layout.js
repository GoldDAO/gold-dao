import { Box } from '@mui/system';
import React from 'react';
import Header from './Header';
import styled from 'styled-components'

const Layout = ({children}) => {
    return (
        <LayoutContainer>
            <Header />
            <Box>
                {children}
            </Box>
        </LayoutContainer>
    );
};

export default Layout;

const LayoutContainer = styled(Box)`
    padding: 82px;
`