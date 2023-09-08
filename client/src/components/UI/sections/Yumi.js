import { Box, Link, Typography } from '@mui/material';
import React from 'react';
import YumiLogo from './../../../../public/images/yumi.svg';
import Image from 'next/image';
import styled from 'styled-components';
import { theme } from '@/theme/theme';

const Yumi = () => {
    return (
        <YumiBanner href="#">
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
    background-color: ${theme.colors.grey};
    border: 1px solid ${theme.colors.black};
    border-radius: 20px;
    align-items: center;
    padding: 40px;
    img {
        grid-column: 1/3;
    }
    div {
        grid-column: 4/11;
    }
`;
