import { Box } from '@mui/system';
import React from 'react';

const SwapCTO = ({data}) => {
    return (
        <Box>
            <Box>{data.title}</Box>
            <Box>
                <button>{data.buttonLabel} {data.inputCurrency}</button>
            </Box>
            <Box>
                <Box>{data.outputCurrency}</Box>
            </Box>
            <Box>{data.value}</Box>
        </Box>
    );
};

export default SwapCTO;