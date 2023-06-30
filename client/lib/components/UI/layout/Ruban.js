import { Box, Typography } from '@mui/material';
import React, { useEffect } from 'react';

const Ruban = () => {

    useEffect(() => {
        console.log('process.env.NETWORK', process.env.NETWORK)
    }, [process.env.NETWORK])
    if (process.env.NETWORK !== 'production') {
        return (
            <Box
                sx={{
                    position: 'fixed',
                    width: '100vw',
                    height: '50px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backgroundColor: '#fff85b',
                    top: 0,
                    left: 0,
                }}
            >
                <Typography>
                    ⚠️ This is a staging environment, Don&apos;t send any real assets here. 💸
                </Typography>
            </Box>
        )
    } else return null;
};

export default Ruban;