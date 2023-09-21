import { IconButton } from '@mui/material';
import React from 'react';
import PermIdentityIcon from '@mui/icons-material/PermIdentity';
import { theme } from '@/theme/theme';

const WalletButton = ({ open, setOpen }) => {
    return (
        <IconButton onClick={() => setOpen(!open)}>
            <PermIdentityIcon sx={{ color: '#D3B872', fill: theme.colors.gold }} />
        </IconButton>
    );
};

export default WalletButton;
