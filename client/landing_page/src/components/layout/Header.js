import { Box, Button, Grid, GridItem } from '@chakra-ui/react';
import React from 'react';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import dynamic from 'next/dynamic';
import WithSubnavigation from './Nav';
import Link from 'next/link';
import GridSystem from './Grid';

const Header = () => {
    const nav = [
        {
            label: 'F.A.Q.',
            href: '/FAQ',
        },
    ];

    return (
        <Box as="header" w={'100%'} pt="40px">
            <GridSystem>
                <GridItem colStart={2} colSpan={2}>
                    <Link href="/">
                        <Image src={Logo} width={50} height={50} alt="gldt-token-logo" />
                    </Link>
                </GridItem>
                <GridItem colSpan={9} display={'grid'} alignContent={'center'}>
                    <WithSubnavigation nav={nav} />
                </GridItem>
            </GridSystem>
        </Box>
    );
};

export default Header;
