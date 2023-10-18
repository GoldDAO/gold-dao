import useGLDTbalance from './../../../../../utils/hooks/useGLDTbalance';
import { useConnect } from '@connect2ic/react';
import React from 'react';
import TokenSign from '../gldt/TokenSign';
import { HStack, Text } from '@chakra-ui/react';

const Balance = () => {
    const { principal } = useConnect();
    const balance = useGLDTbalance(principal);
    return (
        <HStack>
            <Text>{balance}</Text> <TokenSign />
        </HStack>
    );
};

export default Balance;
