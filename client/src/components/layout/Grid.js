import { Container } from '@chakra-ui/react';
import React from 'react';

const Grid = ({ children }) => {
    return (
        <Container
            maxW="4xl"
            centerContent
            sx={{
                display: 'grid',
                gridTemplateColumns: 'repeat(12, 1fr)',
            }}
        >
            {children}
        </Container>
    );
};

export default Grid;
