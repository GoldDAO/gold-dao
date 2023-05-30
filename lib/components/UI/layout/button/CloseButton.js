import { Box } from '@mui/system';
import React from 'react';
import { IconButton } from '@mui/material';
import HighlightOffIcon from '@mui/icons-material/HighlightOff';

const CloseButton = ({ setClose }) => {
    return (
        <IconButton onClick={setClose}><HighlightOffIcon /></IconButton>
    );
};

export default CloseButton;