import { Box, Container } from '@chakra-ui/react';
import React from 'react';

const Grid = ({ children }) => {
    return (
        <Box
            sx={{
                display: 'grid',
                gridTemplateColumns: 'repeat(12, 1fr)',
                width: '100%',
            }}
        >
            {children}
        </Box>
    );
};

export default Grid;
