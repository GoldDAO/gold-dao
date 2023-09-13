import { theme } from '@/theme/theme';
import React from 'react';
import styled from 'styled-components';

const Footer = () => {
    return <FooterContainer></FooterContainer>;
};

export default Footer;

const FooterContainer = styled('footer')`
    height: 250px;
    background-color: ${theme.colors.grey};
    border-top: 1px solid ${theme.colors.gold};
`;
