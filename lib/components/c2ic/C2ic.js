import { ConnectButton, ConnectDialog, useConnect } from '@connect2ic/react';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';

const C2ic = ({ setOpen, open }) => {
    const [, setUser] = useAtom(setGetUserAtom)

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
                console.log('isConnected', isConnected)
                console.log('isConnecting', isConnecting)
                console.log('connected', `activeProvider, ${activeProvider}`)
            },
            onDisconnect: () => {
                // Signed out
            }
        })

    useEffect(() => {
        if (isConnected) {
            console.log("you are connected:", { isConnected, activeProvider, principal })
            setUser({
                principal,
                connect,
                disconnect,
                status,
                isInitializing,
                isIdle,
                isConnecting,
                isConnected,
                isDisconnecting,
                activeProvider
            })
        }
    }, [isConnected]);

    return (
        <Box>
            <ConnectButton
                style={{
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