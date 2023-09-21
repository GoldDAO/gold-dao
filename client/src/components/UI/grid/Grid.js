import { Box } from '@mui/material';
import React from 'react';
import styled from 'styled-components';

const Grid = ({ children }) => {
    return <GridContainer>{children}</GridContainer>;
};

export default Grid;

const GridContainer = styled(Box)`
    display: grid;
    grid-column: span 12;
    grid-template-columns: repeat(12, 1fr);
    gap: 20px;
    row-gap: 120px;
`;

export const ChildGrid = styled(Box)``;
