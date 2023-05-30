import { Dialog, Typography } from '@mui/material';
import { Box } from '@mui/system';
import React, { useState } from 'react';
import styled from 'styled-components';
import CloseButton from './button/CloseButton';
import Address from './Address';

const CustomDialog = ({ content }) => {

    const [open, setOpen] = useState(true)

    return (
        <StyledModal open={open} onClose={() => { }}>
            <StyledModalHead>
                <ModalLabel>
                    <Typography>Select your GLD NFT(s) you want to swap for GLDT</Typography>
                    <CloseButton setClose={() => setOpen(false)} />
                </ModalLabel>
                <Address address={'address'} />
            </StyledModalHead>
            <Box>{content}</Box>
        </StyledModal>
    );
};

export default CustomDialog;

const StyledModal = styled(Dialog)`
  position: fixed;
  z-index: 1300;
  right: 0;
  bottom: 0;
  top: 0;
  left: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 30px;
  .MuiDialog-container{
    width: 100%;
    border-radius: 30px;
  }
  .MuiDialog-paper{
    padding: 40px
  }
  .MuiPaper-root{
    width: 100%;
    max-width: none;
    border-radius: 30px;
  }
`;

const StyledModalHead = styled(Box)`
    display: flex;
    flex-direction: column; 
    padding: 0 0 40px 0;
`

const ModalLabel = styled(Box)`
    width: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    p{
        font-weight: 700;
        width: fit-content;
        font-size: 32px;
    }
`
