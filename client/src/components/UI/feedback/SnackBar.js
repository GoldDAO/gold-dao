import { Alert, Snackbar } from '@mui/material';
import React from 'react';
import styled from 'styled-components';

const SnackBarFeedback = ({ text, open, handleClose }) => {
    return (
        <CustomSnackBar open={open} autoHideDuration={6000} onClose={handleClose}>
            <Alert onClose={handleClose} severity="success">
                {text}
            </Alert>
        </CustomSnackBar>
    );
};

export default SnackBarFeedback;

const CustomSnackBar = styled(Snackbar)`
    .MuiAlert-icon {
        fill: #fff;
    }
`;
