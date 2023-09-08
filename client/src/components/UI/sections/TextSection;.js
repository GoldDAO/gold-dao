import { Box, Typography } from '@mui/material';
import React from 'react';
import styled from 'styled-components';

const TextSection = ({ title, content }) => {
    return (
        <TextSectionContainer>
            <Typography as="h4">{title}</Typography>
            <Typography as="p">{content}</Typography>
        </TextSectionContainer>
    );
};

export default TextSection;

const TextSectionContainer = styled(Box)`
    display: grid;
    grid-template-columns: repeat(10, 1fr);
    grid-column: 2/12;
    gap: 20px;
    h4 {
        grid-column: span 2;
        font-size: 24px;
    }
    p {
        grid-column: 4/11;
        font-size: 20px;
    }
`;
