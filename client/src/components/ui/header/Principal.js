import { useConnect, useWallet } from '@connect2ic/react';
import React, { useEffect } from 'react';
import { HStack, Text } from '@chakra-ui/react';
import Image from 'next/image';
import CopyPrincipal from './CopyPrincipal';

const Principal = () => {
    const { principal } = useConnect();
    const charsCount = 3;
    const firstChars = principal?.slice(0, charsCount);
    const lastChars = principal?.substring(principal.length - charsCount);

    return (
        principal && (
            <HStack>
                <Text>
                    {firstChars}...{lastChars}
                </Text>
                <CopyPrincipal text={principal} />
            </HStack>
        )
    );
};

export default Principal;
