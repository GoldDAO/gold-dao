import Link from 'next/link';
import React from 'react';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';
import { CustomCircularProgress } from '../../styled/common';
import { Box, Typography } from '@mui/material';

const OnGoing = ({ res }) => {
    return (
        <Box>
            {res ? (
                <>
                    {res.map((e, i) => {
                        return e?.map((el, i) => {
                            if (el.ok) {
                                return (
                                    <>
                                        <Box sx={{ display: 'flex' }}>
                                            <CheckCircleIcon />
                                            <Typography>{el.ok?.token_id}</Typography>
                                        </Box>
                                    </>
                                );
                            }
                        });
                    })}
                    <Typography>
                        You can follow the process on{' '}
                        <Link href="/ongoing-swaps">Your ongoing Swap</Link>
                    </Typography>
                </>
            ) : (
                <>
                    <CustomCircularProgress />
                    <Typography>Sending Transaction</Typography>
                </>
            )}
        </Box>
    );
};

export default OnGoing;
