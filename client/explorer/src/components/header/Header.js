import React from 'react';
import Logo from './logo.svg';
import Image from 'next/image';
import { Box, HStack, Input, InputGroup, InputRightElement, Text } from '@chakra-ui/react';
import Link from 'next/link';
import { Search2Icon } from '@chakra-ui/icons';
import SearchBar from './SearchBar';
import Grid from '../Grid';
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
                {/* <Box sx={{ gridColumn: 'span 6' }}>
                    <SearchBar />
                </Box> */}
            </Grid>
        </HStack>
    );
};

export default Header;
