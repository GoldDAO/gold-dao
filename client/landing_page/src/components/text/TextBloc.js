import { Box, Grid, GridItem, Heading, Text } from '@chakra-ui/react';
import Link from 'next/link';
import React from 'react';
import GridSystem from '../layout/Grid';
import { Fade } from 'react-awesome-reveal';

const TextBloc = ({
    title,
    content,
    link,
    subtitle,
    titleSpan,
    textSpan,
    titleOrder,
    colStart,
    colEnd,
    children,
    textOrder,
    childrenSpan,
    variant,
    titleAlign,
    circle,
}) => {
    return (
        <Fade
            as="div"
            style={{
                width: '100%',
            }}
        >
            <Box py={['10px', '10px', 0]}>
                <GridSystem gap={[4, 4, 8, 8, 8]}>
                    <GridItem
                        colSpan={titleSpan}
                        order={titleOrder}
                        colStart={variant ? '' : colStart}
                    >
                        <Box
                            display={'flex'}
                            flexDirection={'column'}
                            justifyContent={'center'}
                            alignItems={'center'}
                            position={'relative'}
                        >
                            <Heading
                                variant="h4"
                                as="h4"
                                textAlign={['left', 'left', 'left', titleAlign]}
                                w={'100%'}
                            >
                                {title}
                            </Heading>
                            {subtitle && (
                                <Text
                                    as="h4"
                                    textAlign={['left', 'left', 'left', titleAlign]}
                                    pt={[0, 0, '20px']}
                                    w={'100%'}
                                >
                                    {subtitle}
                                </Text>
                            )}
                            {link && (
                                <Box
                                    pt={['10px', '10px', '20px']}
                                    textDecoration={'underline'}
                                    w={'100%'}
                                >
                                    <Link href={link.href}>{link.label}</Link>
                                </Box>
                            )}
                            {/* {circle && (
                        <Box
                            position={'absolute'}
                            height={'150px'}
                            width={'150px'}
                            border={'1px'}
                            borderColor={'gold'}
                            zIndex={-1}
                            borderRadius={'50%'}
                        />
                    )} */}
                        </Box>
                    </GridItem>
                    <GridItem
                        colSpan={textSpan}
                        order={textOrder}
                        colEnd={children ? '' : variant ? '' : colEnd}
                        colStart={variant ? colStart : ''}
                    >
                        <Text fontSize={['18px', '18px', '20px']} lineHeight={'26px'}>
                            {content}
                        </Text>
                    </GridItem>
                    {children && (
                        <GridItem
                            colSpan={childrenSpan}
                            order={2}
                            colEnd={variant ? '' : colEnd}
                            colStart={variant ? colStart : [1, 1, 4, 10, 10]}
                        >
                            {children}
                        </GridItem>
                    )}
                </GridSystem>
            </Box>
        </Fade>
    );
};

export default TextBloc;
