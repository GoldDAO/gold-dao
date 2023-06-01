import { Box } from '@mui/system';
import React from 'react';
import { navigation } from '../../../../src/content/navigation';
import Link from 'next/link';
import Image from 'next/image';
import styled from 'styled-components'
import { PrimaryButton } from './button/Buttons';

const Header = () => {
    return (
        <Box as="header">
            <Nav as="nav" sx={{ display: 'flex', justifyContent: 'space-between' }}>
                <Link href="/" style={{ position: 'relative', top: "-45px" }}>
                    <Image width={343} height={200} src="/images/logo.svg" alt="GLDT Logo" />
                </Link>
                <Box as="ul" sx={{ display: 'flex' }}>
                    {navigation.map((e, i) => (
                        <Box key={i} as="li"><Link href={`${e.path}`}>{e.label}</Link></Box>
                    ))}
                    {/* <PrimaryButton>Connect</PrimaryButton> */}
                </Box>
            </Nav>
        </Box>
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