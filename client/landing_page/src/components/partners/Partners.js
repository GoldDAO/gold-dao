import { content } from '@/content';
import { Box, GridItem, Text } from '@chakra-ui/react';
import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import GridSystem from '../layout/Grid';

const Partners = () => {
    const { partners } = content;
    return (
        <Box
            gridColumn={'span 12'}
            wrap={'wrap'}
            justifyContent={'space-between'}
            w={'100%'}
            // borderTop={'1px'}
            // borderBottom={'1px'}
            // py={['20px', '40px']}
            // borderColor={'lightGold'}
        >
            <Text gridColumnStart={2} textAlign={'center'} fontSize={'16px'} alignSelf={'center'}>
                Powered By
            </Text>
            <Box py="20px">
                <GridSystem>
                    {partners.map((e, i) => (
                        <GridItem
                            as={Link}
                            href={e.url}
                            key={i}
                            colSpan={[6, 6, 4, 2]}
                            grid
                            alignSelf={'center'}
                            border="1px"
                            height={'80px'}
                            borderColor={'lightgray'}
                            _hover={{ transform: `scale(1.1)` }}
                            transition={'.2s'}
                            borderRadius={'10px'}
                            display={'flex'}
                            alignItems={'center'}
                            justifyContent={'center'}
                        >
                            <Box>
                                <Image
                                    src={e.img}
                                    alt={`logo ${e.name}`}
                                    style={{
                                        filter: 'grayscale(100%)',
                                        opacity: 1,
                                        width: e.w,
                                    }}
                                />
                            </Box>
                        </GridItem>
                    ))}
                </GridSystem>
            </Box>
        </Box>
    );
};

export default Partners;
