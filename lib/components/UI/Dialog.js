import { Dialog, Typography } from '@mui/material';
import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import MainButton, { CloseButton } from './button/Buttons';
import Address from './Address';
import { useAtom } from 'jotai'
import { cartAtom } from '../../states/atoms/cart';

const CustomDialog = ({ content, open, setOpen, title, address }) => {
    const [cart,] = useAtom(cartAtom)
    return (
        <StyledModal open={open} onClose={() => { }}>
            <StyledModalHead>
                <ModalLabel>
                    <Typography>{title}</Typography>
                    <CloseButton
                        setClose={() => setOpen(false)} />
                </ModalLabel>
                {address &&
                    <Address address={address} />}
            </StyledModalHead>
            <Box>{content}</Box>
            {cart.length > 0 &&
                <Box sx={{ width: '100%', display: 'flex', justifyContent: 'flex-end', padding: '40px 0 20px 0' }}>
                    <MainButton label="Confirm" isInactive={!cart.length} action={() => setOpen(false)} />
                </Box>}
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
  left: 0;§
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
