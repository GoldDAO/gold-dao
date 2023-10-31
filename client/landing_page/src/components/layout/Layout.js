import { Box, Text, VStack } from '@chakra-ui/react';
import React from 'react';
import Nav from '../navigation/Nav';
import Head from 'next/head';
import Link from 'next/link';

const Layout = ({ children, meta }) => {
    const verticalSpacing = ['60px', '80px', '80px', '140px', '180px'];
    const margins = ['20px', '30px', '60px', '140px', '180px'];
    const TextSize = ['18px', '18px', '20px', '22px', '24px'];
    return (
        <>
            <Head>
                <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
                <link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png" />
                <meta name="msapplication-TileColor" content="#da532c" />
                <meta name="theme-color" content="#ffffff" />
            </Head>
            <Nav />
            <Box pb="0">
                {children}
                <Box
                    as="footer"
                    style={{
                        width: '100%',
                        backgroundColor: '#F4F4F4',
                    }}
                    h={['250px', '300px', '400px']}
                >
                    <VStack
                        alignItems={'flex-start'}
                        height={'100%'}
                        justifyContent={'space-between'}
                        py={'40px'}
                        px={margins}
                        fontSize={TextSize}
                    >
                        <VStack alignItems={'flex-start'}>
                            <Link href={'#'}>Explorer</Link>
                            <Link href="/FAQs">FAQ</Link>
                        </VStack>

                        <Text>© 2023 GOLD DAO</Text>
                    </VStack>
                </Box>
            </Box>
        </>
    );
};

export default Layout;
