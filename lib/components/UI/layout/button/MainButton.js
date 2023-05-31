import React from 'react';
import { Button } from '@mui/material';
import styled from 'styled-components';

const MainButton = ({ label, isInactive, action }) => {
    return (
        <StyledButton disabled={isInactive} onClick={action}>
            {label}
        </StyledButton>
    );
};

export default MainButton;

const StyledButton = styled(Button)`
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
`