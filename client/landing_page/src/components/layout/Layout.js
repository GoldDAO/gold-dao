import { Box, Grid, GridItem, Text, VStack } from '@chakra-ui/react';
import React from 'react';
import Nav from '../navigation/Nav';
import Head from 'next/head';
import Link from 'next/link';
import Header from './Header';
import GridSystem from './Grid';

const Layout = ({ children, meta }) => {
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
            <VStack
                w={'100%'}
                alignItems={'flex-start'}
                spacing={['40px', '60px', '100px', '100px']}
            >
                <Header />
                {children}
                <Box as="footer" fontSize={'16px'} w={'100%'} pb="40px">
                    <GridSystem>
                        <GridItem colStart={2} colSpan={2}>
                            <Text fontStyle="italic" fontSize={'̈́18px'}>
                                © 2023 GOLD DAO
                            </Text>
                        </GridItem>
                    </GridSystem>
                </Box>
            </VStack>
        </>
    );
};

export default Layout;
