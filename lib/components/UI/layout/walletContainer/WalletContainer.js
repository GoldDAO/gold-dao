import { Box, Dialog, Typography } from '@mui/material';
import React from 'react';
import styled from 'styled-components';
import Address from '../Address';
import LogoutButton from '../button/Logout';

const WalletContainer = ({ principal, balance, open, setOpen }) => {
    return (
        <WalletBox open={open} setOpen={setOpen}>
            <WalletTitle>Wallet address</WalletTitle>
            <Address address={principal} />
            <BalanceContainer>
                <Typography>GLDT Balance</Typography>
                <Balance>{balance}</Balance>
            </BalanceContainer>
            <LogoutButton />
        </WalletBox>
    );
};

export default WalletContainer;

const WalletTitle = styled(Dialog)`
    font-size: 20px;
    font-weight: 500;
`

const BalanceContainer = styled(Box)`
    background-color: #f4f5f7;
    color: #626263
    border-radius: 8px;
    padding: 12px;
`

const Balance = styled(Typography)`
    font-size: 16px;
    font-weight: 400;
`

const WalletBox = styled(Box)`
    border-radius: 8px;
    box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
    padding: 15px;
`