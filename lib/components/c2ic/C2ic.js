import { ConnectButton, ConnectDialog, useConnect, useDialog } from '@connect2ic/react';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';

const C2ic = () => {
    const [, setUser] = useAtom(setGetUserAtom)
    const { close, isOpen } = useDialog()

    useEffect(() => {
        console.log('isOpen ', isOpen)
    }, [isOpen])

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
                console.log('CONNECTED')
                close()
                document.getElementsByTagName("body")[0].style.overflow = 'scroll'
            },
            onDisconnect: () => {
                console.log('DISCONNECTED')
                setUser({
                    principal: undefined,
                    connect: undefined,
                    disconnect: undefined,
                    status: undefined,
                    isInitializing: undefined,
                    isIdle: undefined,
                    isConnecting: false,
                    isConnected: false,
                    isDisconnecting: undefined,
                    activeProvider: undefined
                })
            }
        })

    useEffect(() => {
        if (isConnected) {
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
            {!isConnected &&
                <>
                    <ConnectButton
                        style={{
                            backgroundColor: "#D3B872",
                            borderRadius: '10px',
                            fontSize: '20px',
                            textTransform: 'uppercase'
                        }}
                    />
                    <ConnectDialog dark={false} />
                </>
            }
        </Box>
    );
};

export default C2ic;