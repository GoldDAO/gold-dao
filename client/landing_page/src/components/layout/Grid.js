import { Grid } from '@chakra-ui/react';
import React from 'react';

const GridSystem = ({ children, gap }) => {
    return (
        <Grid templateColumns="repeat(12, 1fr)" w={'100%'} gap={gap ? gap : 4}>
            {children}
        </Grid>
    );
};

export default GridSystem;
