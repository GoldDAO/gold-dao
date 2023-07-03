import { Box, Typography } from '@mui/material';
import React, { useEffect } from 'react';

const Ruban = () => {

    useEffect(() => {
    }, [process.env.NETWORK])
    if (process.env.NETWORK !== 'production') {
        return (
            <Box
                sx={{
                    position: 'fixed',
                    paddingTop: "3px",
                    width: '100vw',
                    height: '60px',
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