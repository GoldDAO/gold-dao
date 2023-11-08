import React from 'react';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import { GridItem, HStack, Text } from '@chakra-ui/react';
import Link from 'next/link';
import Grid from '@ui/layout/Grid';
import GridSystem from '@ui/layout/GridSystem';

const Header = () => {
    return (
        <HStack as="header" justify={'space-between'} alignItems={'center'}>
            <GridSystem>
                <GridItem colSpan={4}>
                    <Link href={'/'} style={{ gridColumn: 'span 2' }}>
                        <Image src={Logo} width={50} alt="logo" />
                    </Link>
                </GridItem>
                <GridItem alignSelf={'center'} colStart={10}>
                    <Link href="/transparency" style={{ gridColumn: '10/13' }}>
                        <Text fontSize={'18px'}>Transparency</Text>
                    </Link>
                </GridItem>
            </GridSystem>
        </HStack>
    );
};

export default Header;
