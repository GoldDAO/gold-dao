import { ConnectButton, ConnectDialog, useConnect, useDialog } from '@connect2ic/react';
import { Box, Button, Typography } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';
import { appStatusAtom } from '../../states/appStatus';
import GetBalanceOrigynNFTs from '../commands/getBalanceOrigynNFTs';
import { idlFactory } from "./../../../src/agents/declarations/NFTs/GOLD_1g_NFT";
export { idlFactory };

const C2icButton = () => {
    const [, setUser] = useAtom(setGetUserAtom)
    const [appstatus, setAppStatus] = useAtom(appStatusAtom)
    const { close, isOpen } = useDialog()

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

        } else {
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
    }, [isConnected]);

    useEffect(() => {
        setAppStatus(status.idle)
        console.log('appstatus', appstatus)
    }, [status])

    return (
        <Box>
            <>
                {appstatus === 'idle' &&
                    <ConnectButton
                        style={{
                            backgroundColor: "#D3B872",
                            borderRadius: '10px',
                            fontSize: '20px',
                            textTransform: 'uppercase'
                        }}
                    />}
                {appstatus !== 'connected' && appstatus !== 'idle' &&
                    <Typography sx={{
                        color: 'lightgray',
                        fontStyle: 'italic'
                    }}>{appstatus}</Typography>
                }
                <ConnectDialog />
                {isConnected &&
                    <GetBalanceOrigynNFTs />
                }
            </>
        </Box>
    );
};

export default C2icButton;