import { Box, Button, Grid, GridItem } from '@chakra-ui/react';
import React from 'react';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import dynamic from 'next/dynamic';
import WithSubnavigation from './Nav';
import Link from 'next/link';
import GridSystem from './Grid';
import { useEffect } from 'react';
import { useState } from 'react';

const Header = () => {
    const nav = [
        {
            label: 'F.A.Q.',
            href: '/FAQ',
        },
    ];
    const [scrollPosition, setScrollPosition] = useState(0);

    console.log('scrollPosition', scrollPosition);

    const handleScroll = () => {
        setScrollPosition(window.scrollY);
    };

    useEffect(() => {
        window.addEventListener('scroll', handleScroll);
        return () => window.removeEventListener('scroll', handleScroll);
    });
    const height = ['100px', '100px', '100px', '100px', '100px'];
    return (
        <Box>
            <Box height={height}></Box>
            <Box
                position={'fixed'}
                as="header"
                bg={['white', 'white', scrollPosition > 100 ? 'white' : 'transparent']}
                left="0"
                zIndex={10}
                px={['20px', '20px', '40px', '40px', '40px']}
                w={'100%'}
                pt="20px"
                top="0"
                borderBottom={'1px'}
                pb="20px"
                borderColor={scrollPosition > 100 ? 'gold' : 'transparent'}
                transition={'all .4s'}
            >
                <GridSystem>
                    <GridItem colStart={0} colSpan={[2, 2, 2, 1]} order={[3, 3, -1]}>
                        <Link href="/">
                            <Image src={Logo} width={50} height={50} alt="gldt-token-logo" />
                        </Link>
                    </GridItem>
                    <GridItem
                        order={[-1, -1, 3]}
                        colStart={[1, 1, 4]}
                        colSpan={[9, 10, 9]}
                        display={'grid'}
                        alignContent={'center'}
                    >
                        <WithSubnavigation nav={nav} />
                    </GridItem>
                </GridSystem>
            </Box>
        </Box>
    );
};

export default Header;
