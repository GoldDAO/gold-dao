import React from 'react';
import { Button } from '@mui/material';
import styled from 'styled-components';

const MainButton = ({ label }) => {
    return (
        <StyledButton>
            {label}
        </StyledButton>
    );
};

export default MainButton;

const StyledButton = styled(Button)`
    background-color: #313131;
    color: #fff;
    box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
    border-radius: 8px;
`