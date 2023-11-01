import React from 'react';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import { HStack, Text } from '@chakra-ui/react';
import Link from 'next/link';
import Grid from '@ui/layout/Grid';

const Header = () => {
    return (
        <HStack as="header" justify={'space-between'} alignItems={'center'}>
            <Grid>
                <Link href={'/'} style={{ gridColumn: 'span 2' }}>
                    <Image src={Logo} width={50} alt="logo" />
                </Link>
                <Link href="/transparency" style={{ gridColumn: '10/13' }}>
                    <Text>Transparency</Text>
                </Link>
            </Grid>
        </HStack>
    );
};

export default Header;
