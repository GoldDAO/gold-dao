import { content } from '@/content';
import { Box, GridItem, HStack } from '@chakra-ui/react';
import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import GridSystem from '../layout/Grid';

const Partners = () => {
    const { partners } = content;
    return (
        <HStack
            py="20px"
            gridColumn={'span 12'}
            bg="#F8F6EE"
            wrap={'wrap'}
            justifyContent={'space-between'}
            w={'100%'}
        >
            <GridSystem>
                {partners.map((e, i) => (
                    <GridItem key={i} grid colStart={4 + i} alignSelf={'center'}>
                        <Link href={e.url} width={e.w}>
                            <Image
                                src={e.img}
                                alt={`logo ${e.name}`}
                                style={{
                                    filter: 'grayscale(100%)',
                                    opacity: 0.8,
                                    width: e.w,
                                }}
                            />
                        </Link>
                    </GridItem>
                ))}
            </GridSystem>
        </HStack>
    );
};

export default Partners;
