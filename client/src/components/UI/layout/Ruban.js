import { Box, Typography } from '@mui/material';
import React, { useEffect } from 'react';

const Ruban = () => {
  if (process.env.DFX_NETWORK !== 'ic') {
    return (
      <Box
        sx={{
          paddingTop: '3px',
          width: '100vw',
          height: '60px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          backgroundColor: '#fff85b',
          top: 0,
          left: 0,
          zIndex: 20,
        }}
      >
        <Typography>
          ⚠️ This is a staging environment. Don&apos;t send any real assets here. 💸
        </Typography>
      </Box>
    );
  } else return null;
};

export default Ruban;