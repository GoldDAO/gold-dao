import { Box, Typography } from '@mui/material';
import React, { useEffect } from 'react';

const Ruban = () => {
    if (process.env.DFX_NETWORK !== 'ic') {
        return (
            <Box
                sx={{
                    paddingTop: '3px',
                    width: '100vw',
                    height: '40px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backgroundColor: '#ffff85',
                    top: 0,
                    left: 0,
                    zIndex: 20,
                }}
            >
                <Typography sx={{ fontSize: '16px' }}>
                    ⚠️ This is a staging environment. Don&apos;t send any real assets here. 💸
                </Typography>
            </Box>
        );
    } else return null;
};

export default Ruban;
