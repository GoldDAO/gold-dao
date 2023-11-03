import { Box, Divider, GridItem, HStack, Heading, Text } from '@chakra-ui/react';
import React from 'react';
import GridSystem from '../layout/Grid';

const Banner = () => {
    return (
        <GridSystem>
            <GridItem colStart={[1, 1, 1, 2]} colEnd={[12, 12, 12, 7]}>
                <Box>
                    <Heading as="h1" variant="h1">
                        GLDT
                    </Heading>
                    <Heading as="h2" variant="h2">
                        The Future of Tokenized Gold
                    </Heading>
                </Box>
            </GridItem>
            <GridItem colSpan={12}>
                <GridSystem gap={0}>
                    <GridItem colSpan={2} alignSelf={'center'} colStart={[2]}>
                        <Divider orientation="horizontal" borderColor={'black'} />
                    </GridItem>
                    <GridItem colSpan={2} alignSelf={'center'}>
                        <Text w={'100%'} fontSize={'24px'}>
                            Learn how
                            <Box as="span" color="gold">
                                <strong style={{ color: 'inherit' }}>&nbsp;GLDT&nbsp;</strong>
                            </Box>
                            works
                        </Text>
                    </GridItem>
                    <GridItem colSpan={1} alignSelf={'center'}>
                        <Divider orientation="horizontal" borderColor={'black'} />
                    </GridItem>
                </GridSystem>
            </GridItem>
        </GridSystem>
    );
};

export default Banner;
