import { Box, Tooltip } from '@mui/material';
import React from 'react';

const MarketCapContent = ({ marketcap }) => {
    return (
        <Tooltip
            sx={{ fontSize: '1em' }}
            title={<TooltipContent data={{ locked: 'x', unlocked: 'y' }} />}
        >
            <Box style={{ fontSize: '1em', margin: 0 }}>
                &nbsp;
                {Number(marketcap).toLocaleString('en-US')}
                &nbsp;USD
            </Box>
        </Tooltip>
    );
};

export default MarketCapContent;

const TooltipContent = ({ data }) => {
    return (
        <>
            locked: {data.locked} , unlocked: {data.unlocked}
        </>
    );
};
