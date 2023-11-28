import { Box, Button, GridItem, HStack, Heading, Text } from '@chakra-ui/react';
import React, { useEffect, useRef } from 'react';
import GridSystem from '../layout/Grid';
import { Fade } from 'react-awesome-reveal';
import Link from 'next/link';
const Banner = () => {
    const videoRef = useRef();
    useEffect(() => {
        try {
            videoRef.current.play();
        } catch (e) {
            console.log('e', e);
        }
    }, []);
    return (
        <Fade
            as="div"
            style={{
                width: '100%',
            }}
        >
            <Box overflowX={'hidden'} height={'fit-content'}>
                <GridSystem gap={[0, 0, 8]}>
                    <GridItem
                        colSpan={[12, 12, 12, 5, 6]}
                        alignSelf={['center', 'center', 'center', 'flex-start']}
                        justifySelf={['center', 'center', 'center', 'flex-start']}
                        order={[2, 2, 2, -1]}
                    >
                        <video
                            playsinline={true}
                            autoplay={true}
                            autoPlay={true}
                            muted={true}
                            loop={true}
                            ref={videoRef}
                            poster="/images/poster.png"
                            onClick={() => videoRef.current.play()}
                        >
                            <source src="/gldt.mp4" type="video/mp4" />
                            Your browser does not support the video tag.
                        </video>
                    </GridItem>
                    <GridItem
                        colStart={[1, 1, 1, 6, 7]}
                        colEnd={[13, 13]}
                        alignSelf={'center'}
                        justifyContent={['center']}
                    >
                        <Heading
                            as="h1"
                            variant="h1"
                            textAlign={['center', 'center', 'center', 'left']}
                        >
                            GLDT
                        </Heading>
                        <Heading
                            as="h2"
                            variant="h2"
                            textAlign={['center', 'center', 'center', 'left']}
                        >
                            The future of owning physical gold
                        </Heading>
                        <Box
                            width={'100%'}
                            display={'flex'}
                            justifyContent={['center', 'center', 'center', 'flex-start']}
                        >
                            <Button
                                as={Link}
                                variant="yumi"
                                href="/Gold_DAO.pdf"
                                mt="30px"
                                px={['20px', '20px', '40px']}
                            >
                                Read our whitepaper
                            </Button>
                        </Box>
                    </GridItem>
                </GridSystem>
            </Box>
        </Fade>
    );
};

export default Banner;
