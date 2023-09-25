import { ConnectButton, ConnectDialog, useConnect, useDialog, useWallet } from '@connect2ic/react';
import { Box, Button, Typography } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '@/states/user';
import { appStatusAtom } from '@/states/appStatus';
import { emptyAllNftsAtom } from '@/states/nfts';
import { removeAllItemsInCartAtom } from '@/states/cart';
import { theme } from '@/theme/theme';

const C2icButton = () => {
    const [, setUser] = useAtom(setGetUserAtom);
    const [appstatus, setAppStatus] = useAtom(appStatusAtom);
    const { close, isOpen } = useDialog();
    const [wallet] = useWallet();

    const [, emptyAllNfts] = useAtom(emptyAllNftsAtom);
    const [, removeAllCart] = useAtom(removeAllItemsInCartAtom);

    const isReallyConnected = () => {
        if (appstatus === 'connected' && wallet.principal === 'undefined') {
            disconnect();
        }
    };

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
        activeProvider,
    } = useConnect({
        onConnect: () => {
            console.log('CONNECTED');
            close();
            document.getElementsByTagName('body')[0].style.overflow = 'scroll';
        },
        onDisconnect: () => {
            console.log('DISCONNECTED');
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
                activeProvider: undefined,
            });
            removeAllCart();
            emptyAllNfts();
        },
    });

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
                activeProvider,
            });
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
                activeProvider: undefined,
            });
            removeAllCart();
            emptyAllNfts();
        }
    }, [isConnected]);

    useEffect(() => {
        setAppStatus(status.idle);
    }, [status]);

    useEffect(() => {
        if (wallet) {
            isReallyConnected();
        }
    }, [wallet, appstatus]);

    return (
        <Box>
            <>
                {appstatus === 'idle' && (
                    <ConnectButton
                        style={{
                            backgroundColor: 'transparent',
                            color: theme.colors.gold,
                            border: `1px solid ${theme.colors.gold}`,
                            borderRadius: '10px',
                            padding: '10px 30px',
                            fontSize: '20px',
                        }}
                    />
                )}
                {appstatus !== 'connected' && appstatus !== 'idle' && (
                    <Typography
                        sx={{
                            color: 'lightgray',
                            fontStyle: 'italic',
                        }}
                    >
                        {appstatus}
                    </Typography>
                )}
                <ConnectDialog />
            </>
        </Box>
    );
};

export default C2icButton;
