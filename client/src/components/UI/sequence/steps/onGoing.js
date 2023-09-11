import { Check, ClearAll, Savings, ScreenshotMonitor, Assignment } from '@mui/icons-material';
import { Box, CircularProgress, Typography } from '@mui/material';
import Link from 'next/link';
import React from 'react';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';

const OnGoing = ({ res }) => {
    return (
        <Box>
            {res ? (
                res[0].map((e, i) => {
                    return (
                        <>
                            <Box sx={{ display: 'flex' }}>
                                <CheckCircleIcon />
                                <Typography>{e.ok.token_id}</Typography>
                            </Box>
                            <Box>
                                <Typography>
                                    Transaction Sent with success, You can follow the process
                                    on&nbsp;
                                    <Link href="/ongoing-swaps">Ongoing Swaps Page</Link>
                                </Typography>
                            </Box>
                        </>
                    );
                })
            ) : (
                <>
                    <CircularProgress />
                    <Typography>Sending Transaction</Typography>
                </>
            )}
        </Box>
    );
};

export default OnGoing;
