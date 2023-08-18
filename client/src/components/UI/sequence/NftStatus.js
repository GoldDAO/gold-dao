import { CancelsaleButton } from '@/components/UI/button/CancelSale';
import { Box } from '@mui/material';
import React from 'react';
import { useEffect } from 'react';
import { useState } from 'react';
import styled from 'styled-components';

const NftControls = ({ onSale, token_id, weight }) => {
    const [isOnSale, setIsOnSale] = useState(onSale)
    return (
        <Box>
            <NftStatus onSale={onSale} >
                {isOnSale ? 'On Sale' : 'not on sale'}
            </NftStatus>
            {onSale &&
                <CancelsaleButton token_id={token_id} weight={weight} />
            }
        </Box>
    );
};

export default NftControls;

const NftStatus = styled(Box)`
    padding: 5px 8px; 
    background-color: ${props => props.onSale ? `grey` : `grey`};
    color: #fff;
    border-radius: 10px;
    display: inline; 
`