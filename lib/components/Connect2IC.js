import { ConnectButton, ConnectDialog, useConnect } from '@connect2ic/react';
import React from 'react';

const ConnectIC = () => {

    const { isConnected, principal, activeProvider } = useConnect({
        onConnect: () => {
            //
        },
        onDisconnect: () => {
            //
        }
    })

    return (
        <>
            <ConnectButton />
            <ConnectDialog dark={false} />
        </>
    );
};

export default ConnectIC;