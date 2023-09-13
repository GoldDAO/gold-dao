import { Box, Link, Typography } from '@mui/material';
import React from 'react';
import YumiLogo from './../../../../public/images/yumi.svg';
import Image from 'next/image';
import styled from 'styled-components';
import { theme } from '@/theme/theme';

const Yumi = () => {
    return (
        <YumiBanner
            href="https://tppkg-ziaaa-aaaal-qatrq-cai.raw.ic0.app/gold/nfts"
            target="_blank"
        >
            <Image src={YumiLogo} width={200} height={50} alt="Yumi marketplace logo" />
            <Box>
                <Typography>You don’t have any GLDNFT ?</Typography>
                <Typography>Visit Yumi Marketplace</Typography>
            </Box>
        </YumiBanner>
    );
};

export default Yumi;

const YumiBanner = styled(Link)`
    display: grid;
    grid-template-columns: repeat(10, 1fr);
    grid-column: 2/12;
    text-decoration: none;
    color: ${theme.colors.black};
    background-color: ${theme.colors.grey};
    border: 1px solid ${theme.colors.gold};
    border-radius: 20px;
    align-items: center;
    transition: 200ms all;
    padding: 40px;
    img {
        grid-column: 1/3;
    }
    div {
        grid-column: 4/11;
    }
    &:hover {
        box-shadow: 0px 4px 56px 0px rgba(211, 184, 114, 0.2);
        background-color: ${theme.colors.white};
    }
`;
