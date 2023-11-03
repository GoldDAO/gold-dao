import { Box, Button, Divider, GridItem, HStack, Heading, Text } from '@chakra-ui/react';
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
                    <GridItem colSpan={3} alignSelf={'center'}>
                        <HStack fontSize={'20px'}>
                            <Text width={'fit-content'} fontSize={'20px'}>
                                Learn&nbsp;how
                            </Text>
                            <Text fontSize={'20px'} as="span" color="gold" w={'fit-content'}>
                                <strong style={{ color: 'inherit' }}>GLDT</strong>
                            </Text>
                            <Text fontSize={'20px'}>works</Text>
                            <Divider
                                orientation="horizontal"
                                borderColor={'black'}
                                width={'150px'}
                            />
                        </HStack>
                    </GridItem>
                    <GridItem colSpan={2}>
                        <Button variant="yumi" w={'100%'}>
                            Play Video
                        </Button>
                    </GridItem>
                </GridSystem>
            </GridItem>
        </GridSystem>
    );
};

export default Banner;
