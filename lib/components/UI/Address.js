import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import CopyToClipboard from './button/CopyToClipboard';
import {
    Typography,
} from '@mui/material';

const Address = ({ address }) => {

    const firstChars = (str) => str.slice(0, 5);
    const lastChars = (str) => str.slice(-3);

    return (
        <AddressBox>
            <AddressLabel>Wallet address</AddressLabel>
            {address &&
                <>
                    <Typography sx={{ display: 'inline' }}>{firstChars(address) + '...' + lastChars(address)}</Typography>
                    <CopyToClipboard text={address} />
                </>
            }
        </AddressBox>
    );
};

export default Address;

const AddressBox = styled(Box)`
    color: #616161
    font-size: 14px;
`

const AddressLabel = styled(Typography)`
    font-weight: 600;
    color: #616161
`