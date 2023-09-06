import useTotalSupply from '@/components/hooks/useTotalSupply';
import { Typography } from '@mui/material';
import { Box } from '@mui/system';
import React from 'react';
import { useEffect } from 'react';
import styled from 'styled-components';
import MarketCapToolTip from '../tooltip/marketcapTooltip';

const Marketcap = ({ data }) => {
    const totalSupply = useTotalSupply();
    const gprice = 61.78;
    return (
        <Box
            sx={{
                width: '100%',
                display: 'flex',
                justifyContent: 'flex-end',
            }}
        >
            <MarketCapContainer>
                <p style={{ fontWeight: 300 }}>{data.label} : </p>
                <MarketCapToolTip marketcap={(totalSupply * gprice).toFixed(2)} />
            </MarketCapContainer>
        </Box>
    );
};

export default Marketcap;

const MarketCapContainer = styled(Box)`
    font-size: 1em;
    background-color: #f7f7f7;
    width: fit-content;
    padding: 20px;
    border-radius: 10px;
    display: flex;
    @media (max-width: 940px) {
        font-size: 0.8em;
    }
    @media (max-width: 540px) {
        width: 100%;
    }
`;
