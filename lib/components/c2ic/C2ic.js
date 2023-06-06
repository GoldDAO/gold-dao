import { ConnectButton, ConnectDialog, useConnect } from '@connect2ic/react';
import { Box } from '@mui/material';
import React from 'react';

const C2ic = () => {


    const {
        principal,
        connect,
        disconnect,
        status,
        isInitializing,
        isIdle,
        isConnecting,
        isConnected,
        isDisconnecting,
        activeProvider, } = useConnect({
            onConnect: () => {
                console.log('connected', `principal, ${principal}`)
            },
            onDisconnect: () => {
                // Signed out
            }
        })


    return (
        <Box>
            <ConnectButton style={{
                backgroundColor: "#D3B872",
                borderRadius: '10px',
                fontSize: '20px',
                textTransform: 'uppercase'
            }}
            />
            <ConnectDialog dark={false} />
        </Box>
    );
};

export default C2ic;