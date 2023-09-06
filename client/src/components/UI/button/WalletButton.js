import { IconButton } from '@mui/material';
import React from 'react';
import PermIdentityIcon from '@mui/icons-material/PermIdentity';

const WalletButton = ({ open, setOpen }) => {
    return (
        <IconButton onClick={() => setOpen(!open)}>
            <PermIdentityIcon sx={{ color: '#D3B872' }} />
        </IconButton>
    );
};

export default WalletButton;