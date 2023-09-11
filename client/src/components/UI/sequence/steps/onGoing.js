import { Check, ClearAll, Savings, ScreenshotMonitor, Assignment } from '@mui/icons-material';
import { Box, CircularProgress } from '@mui/material';
import Link from 'next/link';
import React from 'react';

const OnGoing = ({ res }) => {
    console.log('res', res);
    return <Box>{res ? res[0].map(() => <div>item</div>) : <CircularProgress />}</Box>;
};

export default OnGoing;
