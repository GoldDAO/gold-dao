import React from 'react';
import Logo from './logo.svg';
import Image from 'next/image';
import { Box, HStack, Input, InputGroup, InputRightElement, Text } from '@chakra-ui/react';
import Link from 'next/link';
import { Search2Icon } from '@chakra-ui/icons';
import SearchBar from './SearchBar';
const Header = () => {
    return (
        <HStack as="header" justify={'space-between'}>
            <Link href={'/'}>
                <Image src={Logo} width={50} alt="logo" />
            </Link>
            <SearchBar />
            {/* <Link href="/transparency">
                <Text>Transparency</Text>
            </Link> */}
        </HStack>
    );
};

export default Header;
