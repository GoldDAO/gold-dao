import React from 'react';
import { Box, Button, Divider, GridItem, HStack, Heading, Text } from '@chakra-ui/react';
import Logo from '/public/images/yumi.png';
import GridSystem from '../layout/Grid';
import Image from 'next/image';
import { Fade } from 'react-awesome-reveal';
const Yumi = () => {
    return (
        <Box
            w={'100%'}
            py={['20px', '20px', '40px', '60px', '80px']}
            borderTop={'1px'}
            borderBottom={'1px'}
            borderColor={'lightGold'}
            margin="0 auto"
            // borderRadius={'10px'}
            // bg="bg"
            maxWidth="1540px"
        >
            <Fade>
                <GridSystem gap={[4, 4, 6, 6, 8]}>
                    <GridItem
                        colStart={[2, 4, 4, 2, 2]}
                        colSpan={[10, 6, 6, 2, 2]}
                        order={[2, 2, 2, 1, 1]}
                    >
                        <Heading variant={'h4'} as="h4" textAlign={['center', 'center', 'left']}>
                            Don’t own any{' '}
                            <Box color="gold" as="span" fontWeight={600}>
                                GLD NFTs
                            </Box>
                            &nbsp;?
                        </Heading>
                    </GridItem>
                    <GridItem
                        colSpan={[3, 3, 3, 2, 2]}
                        colStart={[1, 1, 1, 5, 5]}
                        order={[1, 1, 1, 2, 2]}
                        display={['none', 'block']}
                    >
                        <Box
                            height={0}
                            position={'relative'}
                            top={['-20px', , , , '-55px']}
                            right={['-20px', '-20px', '-20px', 0, 0]}
                            mixBlendMode="multiply"
                        >
                            <Image src={Logo} alt="Gold 3D Yumi Logo" />
                        </Box>
                    </GridItem>
                    <GridItem
                        colSpan={[10, 8, 8, 5, 5]}
                        order={[3, 3, 3, 3, 3]}
                        colStart={[2, 4, 4, 7, 7]}
                    >
                        <Text textAlign={['center', 'center', 'left']}>
                            join the global movement towards a more transparent and accessible
                            buying and selling of gold.
                        </Text>
                        <Button
                            px="50px"
                            mt="20px"
                            variant={'yumi'}
                            width={['100%', '100%', 'fit-content']}
                        >
                            Buy GLD NFTs
                        </Button>
                    </GridItem>
                </GridSystem>
            </Fade>
        </Box>
    );
};

export default Yumi;
