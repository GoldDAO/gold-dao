import React from 'react';
import Logo from './logo.svg';
import Image from 'next/image';
import { Box, HStack, Text } from '@chakra-ui/react';
import Link from 'next/link';
const Header = () => {
    return (
        <HStack as="header" justify={'space-between'}>
            <Link href={'/'}>
                <Image src={Logo} width={50} alt="logo" />
            </Link>
            <Link href="/transparency">
                <Text>Transparency</Text>
            </Link>
        </HStack>
    );
};

export default Header;
