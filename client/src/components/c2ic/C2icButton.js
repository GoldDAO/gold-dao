import { ConnectButton, ConnectDialog, useConnect, useDialog, useWallet } from '@connect2ic/react';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { emptyAllNftsAtom } from '@/atoms/nfts';
import { removeAllItemsInCartAtom } from '@/atoms/cart';

const C2icButton = () => {
    const [, emptyAllNfts] = useAtom(emptyAllNftsAtom);
    const [, removeAllCart] = useAtom(removeAllItemsInCartAtom);

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
            emptyAllNfts();
        },
    });

    return (
        <>
            <ConnectButton></ConnectButton>
            <ConnectDialog />
        </>
    );
};

export default C2icButton;
