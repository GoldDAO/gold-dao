import React, { useState } from 'react';
import { setGetUserAtom } from '../../../states/user';
import WalletContainer from './WalletContainer';
import WalletButton from '../button/WalletButton';
import { useAtom } from 'jotai';

const Wallet = () => {
    const [user,] = useAtom(setGetUserAtom)
    const [open, setOpen] = useState(false)

    return (
        <>
            <WalletButton
                open={open}
                setOpen={setOpen}
            />
            <WalletContainer
                open={open}
                setOpen={setOpen}
                user={user}
            />
        </>
    );
};

export default Wallet;