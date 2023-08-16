import { Box } from '@mui/system';
import React, { useEffect, useState } from 'react';
import { navigation } from '../../../../src/content/navigation';
import Link from 'next/link';
import Image from 'next/image';
import styled from 'styled-components'
import { useAtom } from 'jotai';
import { setGetUserAtom } from '../../../states/user';
import dynamic from 'next/dynamic';
import { appStatusAtom } from '../../../states/appStatus';
import { Typography } from '@mui/material';
import Ruban from './Ruban';

const C2icButton = dynamic(() => import('./../../c2ic/C2icButton'), {
    ssr: false,
});
const Wallet = dynamic(() => import('./../wallet/Wallet'), {
    ssr: false,
});


const Header = () => {
    const [user,] = useAtom(setGetUserAtom)
    const [status,] = useAtom(appStatusAtom)


    return (
        <HeaderContainer as="header" >
            <Ruban />
            <Nav as="nav" sx={{ display: 'flex', justifyContent: 'space-between', }}>
                <Link href="/" >
                    <Image width={190} height={80} src="/images/logo.svg" alt="GLDT Logo" />
                </Link>
                <Box as="ul" sx={{ display: 'flex' }}>
                    {navigation.map((e, i) => (
                        <Box key={i} as="li"><Link href={`${e.path}`}>{e.label}</Link></Box>
                    ))}
                </Box>
            </Nav>
            <Box sx={{ position: 'relative' }}>
                {status === 'connected' &&
                    <Wallet />}
                < C2icButton />
            </Box>
        </HeaderContainer>
    );
};

export default Header;

const Nav = styled('nav')`
    display: flex;
    justify-content: space-between;
    ul{
        li{
            padding: 0 20px;
            text-decoration: none;
            list-style: none;
        }
    }
`

const HeaderContainer = styled('header')`
    display: flex;
    padding-bottom: 20px;
    border-bottom: 1px solid #D3B872;
    justify-content: space-between;
    align-items: center;

`
