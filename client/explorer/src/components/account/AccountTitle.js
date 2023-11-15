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
                <Text
                    maxW={['200px', '200px', '600px', '800px']}
                    color={'black'}
                    fontSize={'20px'}
                    textOverflow={'ellipsis'}
                    overflow={'hidden'}
                    whiteSpace={'nowrap'}
                    width={'100%'}
                >
                    {data.id}
                </Text>
                <CopyPrincipal text={data.id} />
            </HStack>
        </VStack>
    );
};

export default AccountTitle;
