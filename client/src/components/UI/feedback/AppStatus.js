import { userAtom } from '@/states/user';
import { Box, CircularProgress, Typography } from '@mui/material';
import { useAtom } from 'jotai';
import React from 'react';
import { useEffect } from 'react';
import { CustomCircularProgress } from '../styled/common';

const AppStatus = () => {
    const { isConnecting, isDisconnecting, isIdle, isInitializing, isConnected } =
        useAtom(userAtom);
    const user = useAtom(userAtom);
    console.log('user', user);
    return (
        <Box sx={{ gridColumn: 'span 12' }}>
            {isIdle ||
                isInitializing ||
                isDisconnecting ||
                (isConnecting && (
                    <>
                        <CustomCircularProgress />
                        <Typography>
                            {isIdle && 'Idle'}
                            {isInitializing && 'Initializing'}
                            {isDisconnecting && 'Disconnecting'}
                            {isConnecting && 'Connecting'}
                        </Typography>
                    </>
                ))}
            {!isConnected && !isInitializing && (
                <Typography sx={{ width: '100%' }}>Please connect your wallet</Typography>
            )}
        </Box>
    );
};

export default AppStatus;
