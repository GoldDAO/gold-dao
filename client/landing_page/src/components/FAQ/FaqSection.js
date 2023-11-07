import { Accordion, Box, GridItem, Heading } from '@chakra-ui/react';
import React from 'react';
import GridSystem from '../layout/Grid';
import Link from 'next/link';
import Question from './Question';
import Q from './content.json';
import { Fade } from 'react-awesome-reveal';

const FaqSection = ({ full }) => {
    console.log('full', full);
    return (
        <Box width={'100%'}>
            <Fade>
                <GridSystem>
                    <GridItem
                        colStart={[1, 1, 1, 2]}
                        colSpan={[12, 12, 3, 2]}
                        borderTop={'1px'}
                        borderTopColor={'lightGold'}
                    >
                        <Box pt={['20px']}>
                            <Heading variant={'h4'} as={full ? 'h1' : 'h4'}>
                                Frequently Asked Question
                            </Heading>
                            {!full && (
                                <Box mt={[0, 0, '20px']} textDecoration={'underline'}>
                                    <Link href={'FAQ'}>View more FAQs</Link>
                                </Box>
                            )}
                        </Box>
                    </GridItem>
                    <GridItem colSpan={[12, 12, 8]}>
                        <Accordion allowToggle>
                            {Q.map((e, i) => {
                                if (!full && i < 3) {
                                    return <Question key={i} q={e.q} r={e.r} />;
                                } else if (full) {
                                    return <Question key={i} q={e.q} r={e.r} />;
                                }
                            })}
                        </Accordion>
                    </GridItem>
                </GridSystem>
            </Fade>
        </Box>
    );
};

export default FaqSection;
