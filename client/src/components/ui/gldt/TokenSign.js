import React from 'react';
import Logo from '/public/images/logo.svg';
import { HStack, Text } from '@chakra-ui/react';
import Image from 'next/image';

const TokenSign = () => {
    return (
        <HStack>
            <Image src={Logo} width={20} height={20} alt="gldt-token-logo" />
            <Text
                borderRadius={'20px'}
                sx={{
                    fontWeight: 500,
                    fontSize: '0.75em',
                }}
            >
                GLDT
            </Text>
        </HStack>
    );
};

export default TokenSign;
