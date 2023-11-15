import React from 'react';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import { GridItem, HStack, Text } from '@chakra-ui/react';
import Link from 'next/link';
import GridSystem from '@ui/layout/GridSystem';

const Header = () => {
    return (
        <HStack
            as="header"
            justify={'space-between'}
            alignItems={'center'}
            mb={['20px', '20px', '40px']}
        >
            <GridSystem>
                <GridItem colSpan={4}>
                    <Link href={'/'} style={{ gridColumn: 'span 2' }}>
                        <Image src={Logo} width={40} alt="logo" />
                    </Link>
                </GridItem>
                <GridItem alignSelf={'center'} colStart={[12, 12, 12]}>
                    <Link href="/transparency">
                        <Text fontSize="16px">Transparency</Text>
                    </Link>
                </GridItem>
            </GridSystem>
        </HStack>
    );
};

export default Header;
