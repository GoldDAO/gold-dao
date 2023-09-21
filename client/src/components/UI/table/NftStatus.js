import { CancelsaleButton } from '@/components/UI/button/CancelSale';
import { Box } from '@mui/material';
import React from 'react';
import { useEffect } from 'react';
import { useState } from 'react';
import styled from 'styled-components';
import SnackBarFeedback from '../feedback/SnackBar';

const NftControls = ({ onSale, token_id, weight }) => {
    const [isOnSale, setIsOnSale] = useState(onSale ? true : false);
    const [feedback, setFeedback] = useState(false);

    const handleCloseFeedback = () => {
        setFeedback(false);
    };

    return (
        <Box sx={{ display: 'flex', justifyContent: 'flex-start', alignItems: 'center' }}>
            <NftStatus>{isOnSale ? 'On Sale' : 'not on sale'}</NftStatus>
            {isOnSale && (
                <CancelsaleButton
                    setFeedback={setFeedback}
                    token_id={token_id}
                    weight={weight}
                    setIsOnSale={setIsOnSale}
                />
            )}
            <>
                <SnackBarFeedback
                    handleClose={handleCloseFeedback}
                    open={feedback}
                    text={'Sale Canceled'}
                />
            </>
        </Box>
    );
};

export default NftControls;

const NftStatus = styled(Box)`
    padding: 5px 8px;
    background-color: grey;
    color: #fff;
    border-radius: 10px;
    display: inline;
`;
