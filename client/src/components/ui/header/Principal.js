import { useConnect, useWallet } from '@connect2ic/react';
import React, { useEffect } from 'react';
import { HStack, Text } from '@chakra-ui/react';
import Image from 'next/image';

const Principal = () => {
    const { principal } = useConnect();
    const charsCount = 3;
    const firstChars = principal.slice(0, charsCount);
    const lastChars = principal.substring(principal.length - charsCount);
    const [wallet] = useWallet();
    console.log('wallet', wallet);

    useEffect(() => {
        console.log('wallet', wallet);
    }, [wallet]);
    return (
        <HStack>
            <Text>
                {firstChars}...{lastChars}
            </Text>
        </HStack>
    );
};

export default Principal;
