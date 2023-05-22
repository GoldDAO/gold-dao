import { Box } from '@mui/system';
import React from 'react';

const Marketcap = ({data}) => {
    return (
        <Box>
            <Box>{data.label}</Box>
            <Box>{data.value}</Box>
        </Box>
    );
};

export default Marketcap;