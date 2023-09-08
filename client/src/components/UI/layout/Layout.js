import { Box } from '@mui/system';
import React from 'react';
import Header from './Header';
import styled from 'styled-components';
import Grid from '../grid/Grid';
import Footer from './Footer';

const Layout = ({ children }) => {
    return (
        <>
            <LayoutContainer>
                <Grid>
                    <Header />
                    <ContentContainer>
                        <Grid>{children}</Grid>
                    </ContentContainer>
                </Grid>
            </LayoutContainer>
            <Footer />
        </>
    );
};

export default Layout;

const LayoutContainer = styled(Box)`
    padding: 40px 60px;
    @media (max-width: 1140px) {
        padding: 30px 60px;
    }
    @media (max-width: 840px) {
        padding: 20px 40px;
    }
    @media (max-width: 480px) {
        padding: 10px 20px;
    }
`;
const ContentContainer = styled(Box)`
    grid-column: span 12;
    padding-top: 40px;
    @media (max-width: 1140px) {
        padding-top: 30px;
    }
    @media (max-width: 840px) {
        padding-top: 20px;
    }
    @media (max-width: 480px) {
        padding-top: 10px;
    }
`;
