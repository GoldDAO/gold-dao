import dynamic from 'next/dynamic';
import React from 'react';
import Logo from '/public/images/logo.svg';
import Image from 'next/image';
import C2icButton from '../../c2ic/C2icButton';
import { useConnect } from '@connect2ic/react';
import Balance from './Balance';
import Principal from './Principal';
import Link from 'next/link';

const Header = () => {
    const { isConnected } = useConnect();
    return (
        <header>
            <Image src={Logo} width={50} height={50} />
            {isConnected && (
                <>
                    <Link href="/my-account">My Account</Link> <Principal /> <Balance />
                </>
            )}
            <C2icButton />
        </header>
    );
};

export default Header;
