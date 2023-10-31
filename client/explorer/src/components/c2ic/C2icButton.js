import { ConnectButton, ConnectDialog, useConnect, useDialog, useWallet } from '@connect2ic/react';
import React, { useEffect } from 'react';
import { useRouter } from 'next/router';

const C2icButton = () => {
    const router = useRouter();
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
            document.getElementsByTagName('body')[0].style.overflow = 'scroll';
        },
        onDisconnect: () => {
            console.log('DISCONNECTED');
            removeAllCart();
            router.push('/');
            emptyAllNfts();
        },
    });

    return (
        <>
            <ConnectButton
                style={{ border: '1px solid #000', color: '#000', backgroundColor: '#fff' }}
            ></ConnectButton>
            <ConnectDialog />
        </>
    );
};

export default C2icButton;
