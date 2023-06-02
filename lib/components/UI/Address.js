import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import CopyToClipboard from './button/CopyToClipboard';
import {
    Typography,
} from '@mui/material';

const Address = ({ address }) => {
    return (
        <AddressBox>
            <AddressLabel>Wallet address</AddressLabel>
            <Typography sx={{ display: 'inline' }}>{address}</Typography>
            <CopyToClipboard text={address} />
        </AddressBox>
    );
};

export default Address;

const AddressBox = styled(Box)`
    color: #616161
    font-size: 14px;
    padding: 40px 0 0 0;
`

const AddressLabel = styled(Typography)`
    font-weight: 600;
    color: #616161
`