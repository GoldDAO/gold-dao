import { Box, GridItem, Text, VStack } from '@chakra-ui/react';
import React from 'react';
import Header from './Header';
import GridSystem from './Grid';
import Metas from '@ui/layout/Metas';

const Layout = ({ children, meta }) => {
    return (
        <>
            <Metas meta={meta} />
            <VStack
                // maxW={'1540px'}
                px={['20px', '20px', '40px']}
                margin={'0 auto'}
                w={'100%'}
                alignItems={'flex-start'}
                spacing={['40px', '60px', '100px', '100px']}
            >
                <Header />
                {children}
                <Box as="footer" fontSize={'16px'} w={'100%'} pb="40px">
                    <GridSystem>
                        <GridItem colStart={0} colSpan={10}>
                            <Text fontSize={'12px'}>© 2023 GOLD DAO</Text>
                        </GridItem>
                    </GridSystem>
                </Box>
            </VStack>
        </>
    );
};

export default Layout;
