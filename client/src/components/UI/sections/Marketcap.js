import { Typography } from '@mui/material';
import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';

const Marketcap = ({ data }) => {
    return (
        <Box sx={{
            width: '100%',
            display: 'flex',
            justifyContent: 'flex-end'
        }}>
            <MarketCapContainer>
                <p style={{ fontWeight: 300 }}>{data.label} : </p>
                <p style={{ color: "#D3B872" }}>&nbsp;{data.value}</p>
            </MarketCapContainer>
        </Box>

    );
};

export default Marketcap;

const MarketCapContainer = styled(Box)`
    font-size: 1em;
    background-color: #F7F7F7;
    width: fit-content;
    padding: 20px;
    border-radius: 10px;
    display: flex;
    @media (max-width: 940px){
        font-size: .8em;
    }
    @media (max-width: 540px){
        width: 100%;
    }
`