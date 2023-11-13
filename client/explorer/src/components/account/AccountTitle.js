import { Box, HStack, Text, VStack } from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';
import React from 'react';

const AccountTitle = ({ data }) => {
    const charsCount = 3;
    const firstChars = data.id?.slice(0, charsCount);
    const lastChars = data.id?.substring(data.id.length - charsCount);

    return (
        <VStack alignItems={'flex-start'} spacing={'0'} w={'100%'}>
            <Text color={'blackAlpha.600'} fontSize={'14px'}>
                {data.label}
            </Text>
            <HStack>
                <Text color={'black'} fontSize={'20px'}>
                    {firstChars}...{lastChars}
                </Text>
                <CopyPrincipal text={data.id} />
            </HStack>
        </VStack>
    );
};

export default AccountTitle;
