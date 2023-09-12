import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import CopyToClipboard from '../button/CopyToClipboard';
import { Typography } from '@mui/material';

const Address = ({ address, copyBtn }) => {
    const firstChars = (str) => str.slice(0, 5);
    const lastChars = (str) => str.slice(-3);

    return (
        <AddressBox>
            <AddressLabel>Connected Principal</AddressLabel>
            {address && (
                <>
                    <Typography sx={{ display: 'inline', paddingLeft: '20px' }}>
                        {firstChars(address) + '...' + lastChars(address)}
                    </Typography>
                    {copyBtn && <CopyToClipboard text={address} />}
                </>
            )}
        </AddressBox>
    );
};

export default Address;

const AddressBox = styled(Box)`
    color: #616161;
    display: flex;
    align-items: center;
    font-size: 14px;
`;

const AddressLabel = styled(Typography)`
    font-weight: 400;
    color: #616161;
`;
