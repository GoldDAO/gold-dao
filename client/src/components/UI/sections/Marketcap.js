import useTotalSupply from '@/components/hooks/useTotalSupply';
import { Typography } from '@mui/material';
import { Box } from '@mui/system';
import React from 'react';
import { useEffect } from 'react';
import styled from 'styled-components';

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
                <p style={{ color: '#D3B872' }}>&nbsp;{(totalSupply * gprice).toFixed(2)} USD</p>
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
