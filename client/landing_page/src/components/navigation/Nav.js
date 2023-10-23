import { Box, Button, HStack, Text } from '@chakra-ui/react';
import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import Logo from '/public/images/logo.svg';

const Nav = () => {
    const headerMargins = ['20px', '30px', '49px', '49px', '49px'];
    const TextSize = ['18px', '18px', '20px', '22px', '24px'];

    return (
        <Box
            pt={'34px'}
            as="header"
            px={headerMargins}
            style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
            }}
        >
            <Image width={80} src={Logo} alt="logo" />
            <HStack spacing="50px">
                <Link href="/FAQ">
                    <Text fontSize={TextSize}>FAQ</Text>
                </Link>
                <Button
                    as="a"
                    _hover={{
                        bg: '#D3B872',
                    }}
                    target="_blank"
                    href="https://app.gldt.org"
                    py={'40px'}
                    px="25px"
                    bg="#D3B872"
                    borderRadius={'30px'}
                    fontSize={('18px', '24px')}
                    fontWeight={'bold'}
                    color={'#fff'}
                >
                    Use GLDT
                </Button>
            </HStack>
        </Box>
    );
};

export default Nav;
