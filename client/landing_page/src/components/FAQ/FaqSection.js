import { Accordion, Box, GridItem, Heading } from '@chakra-ui/react';
import React from 'react';
import GridSystem from '../layout/Grid';
import Link from 'next/link';
import Question from './Question';
import { Fade } from 'react-awesome-reveal';
import { faq } from './faq';

const FaqSection = ({ full }) => {
    console.log('full', full);
    return (
        <Box width={'100%'}>
            <GridSystem>
                <GridItem
                    colStart={[1, 1, 1, 2]}
                    colSpan={[11, 11, 3, 2]}
                    borderTop={'1px'}
                    borderTopColor={'lightGold'}
                >
                    <Box pt={['20px']}>
                        <Heading variant={'h4'} as={full ? 'h1' : 'h4'}>
                            Frequently Asked Question
                        </Heading>
                        {!full && (
                            <Box mt={[0, 0, '20px']} textDecoration={'underline'}>
                                <Link pt="7px" href={'FAQ'}>
                                    View more FAQs
                                </Link>
                            </Box>
                        )}
                    </Box>
                </GridItem>
                <GridItem colSpan={[11, 11, 8]}>
                    <Accordion allowToggle>
                        {faq.map((e, i) => {
                            if (!full && i < 3) {
                                return (
                                    <Fade>
                                        <Question key={i} q={e.q} r={e.r} />
                                    </Fade>
                                );
                            } else if (full) {
                                return (
                                    <Fade>
                                        <Question key={i} q={e.q} r={e.r} />
                                    </Fade>
                                );
                            }
                        })}
                    </Accordion>
                </GridItem>
            </GridSystem>
        </Box>
    );
};

export default FaqSection;
