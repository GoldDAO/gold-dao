import { Box, HStack } from '@chakra-ui/react';
import React from 'react';
import TokenSign from '@ui/gldt/TokenSign';

const GldtValue = () => {
    return (
        <HStack
            justify={'flex-end'}
            pt="20px"
            borderTop="1px"
            borderColor={'gold'}
            spacing={'10px'}
        >
            <Box
                top="2px"
                h="40px"
                border="2px"
                w={'31px'}
                borderColor={'gold'}
                p="16px 6px"
                transform={'rotate(15deg)'}
                bg="white"
                position="relative"
                zIndex="3"
                _before={{
                    position: 'absolute',
                    h: '38px',
                    right: '-5px',
                    top: '-9px',
                    zIndex: -10,
                    w: '25px',
                    border: '2px',
                    borderColor: 'gold',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    bg: 'white',
                    fontSize: '16px',
                    color: 'darkGold',
                    content: "'1g'",
                }}
            ></Box>
            <Box pl="10px">=</Box>
            <Box
                p="5px"
                fontSize={'16px'}
                borderRadius="50%"
                border={'2px'}
                borderColor={'gold'}
                color={'darkGold'}
            >
                100
            </Box>
            <Box>
                <TokenSign />
            </Box>
        </HStack>
    );
};

export default GldtValue;
