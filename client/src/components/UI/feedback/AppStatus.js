import { userAtom } from '@/states/user';
import { Box, CircularProgress } from '@mui/material';
import { useAtom } from 'jotai';
import { initialize } from 'next/dist/server/lib/render-server';
import React from 'react';
import { useEffect } from 'react';
import { CustomCircularProgress } from '../styled/common';

const AppStatus = () => {
    const { isConnecting, isDisconnecting, isIdle, isInitializing } = useAtom(userAtom);
    return (
        <Box>
            <CustomCircularProgress />
            <>
                {isIdle && 'Idle'}
                {isInitializing && 'Initializing'}
                {isDisconnecting && 'Disconnecting'}
                {isConnecting && 'Connecting'}
            </>
        </Box>
    );
};

export default AppStatus;
