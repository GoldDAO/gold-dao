import React from 'react';
import { Button } from '@mui/material';
import styled from 'styled-components';
import { IconButton } from '@mui/material';
import HighlightOffIcon from '@mui/icons-material/HighlightOff';
import { Children } from 'react';
import { theme } from '@/theme/theme';

const MainButton = ({ label, isInactive, action, style, children }) => {
    return (
        <StyledButton disabled={isInactive} onClick={action} style={style}>
            {label && label}
            {children && children}
        </StyledButton>
    );
};

export default MainButton;

const StyledButton = styled(Button)`
    &.MuiButton-root {
        text-transform: capitalize;
        background-color: ${theme.colors.gold};
        color: #fff;
        padding: 8px 40px;
        border-radius: 8px;
        &:hover {
        }
    }
`;

export const CloseButton = ({ setClose }) => {
    return (
        <IconButton onClick={setClose}>
            <HighlightOffIcon />
        </IconButton>
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
        background-color: #d3b872;
        color: #fff;
        border-radius: 10px;
        font-size: 1em;
        border: 0;
        cursor: pointer;
        outline: none;
        box-shadow: none;
        &:hover {
            box-shadow: none;
            background-color: #d3b872;
        }
    }
`;
