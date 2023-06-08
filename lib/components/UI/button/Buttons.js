import React from 'react';
import { Button } from '@mui/material';
import styled from 'styled-components';
import { IconButton } from '@mui/material';
import HighlightOffIcon from '@mui/icons-material/HighlightOff';

const MainButton = ({ label, isInactive, action }) => {
    return (
        <StyledButton disabled={isInactive} onClick={action}>
            {label}
        </StyledButton>
    );
};

export default MainButton;

const StyledButton = styled(Button)`
    &.MuiButton-root {
        max-width: 289px;
        min-width: 289px;
        text-transform: capitalize;
        background-color: #313131;
        color: #fff;
        border: 1px solid #313131;
        box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
        border-radius: 8px;
        &:hover{
            box-shadow: none;
            border: 1px solid #5b5858;
            background-color: #313131;
        }
    }

`

export const CloseButton = ({ setClose }) => {
    return (
        <IconButton onClick={setClose}><HighlightOffIcon /></IconButton>
    );
};

export const PrimaryButton = ({ label, isInactive, action }) => {
    return (
        <StyledPrimaryButton disabled={isInactive} onClick={action}>
            {label}
        </StyledPrimaryButton>
    );
};

export const StyledPrimaryButton = styled(Button)`
    &.MuiButton-root {
        height: fit-content;
        padding: 9px 25px 5px 25px;
        background-color: #D3B872;
        color: #fff;
        border-radius: 10px;
        font-size: 1em;
        border: 0;
        cursor: pointer;
        outline: none;
        box-shadow: none;
        &:hover{
            box-shadow: none;
            background-color: #D3B872;
        }
    }
`

