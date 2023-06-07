import { Box } from '@mui/system';
import React, { useEffect, useState } from 'react';
import { navigation } from '../../../../src/content/navigation';
import Link from 'next/link';
import Image from 'next/image';
import styled from 'styled-components'
import C2ic from '../../c2ic/C2ic';
import { useWallet } from '@connect2ic/react';
import WalletContainer from './walletContainer/WalletContainer';


const Header = () => {
    const [user,] = useAtom(setGetUserAtom)
    const [open, setOpen] = useState(false)
    return (
        <HeaderContainer as="header">
            <Nav as="nav" sx={{ display: 'flex', justifyContent: 'space-between' }}>
                <Link href="/" style={{ position: 'relative', top: "-45px" }}>
                    <Image width={343} height={200} src="/images/logo.svg" alt="GLDT Logo" />
                </Link>
                <Box as="ul" sx={{ display: 'flex' }}>
                    {navigation.map((e, i) => (
                        <Box key={i} as="li"><Link href={`${e.path}`}>{e.label}</Link></Box>
                    ))}
                </Box>
            </Nav>
            <C2ic
                setOpen={setOpen}
            />
            {user?.isConnected &&
                <WalletContainer
                    open={open}
                    setOpen={setOpen}
                    address={'user'}
                    balance={user.principal}
                />
            }
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
    justify-content: space-between;

`
