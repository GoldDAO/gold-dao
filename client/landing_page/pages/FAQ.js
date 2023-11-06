import Question from '@/components/FAQ/Question';
import Layout from '@/components/layout/Layout';
import { Accordion, Box, GridItem, Heading } from '@chakra-ui/react';
import React from 'react';
import Q from './../src/components/FAQ/content.json';
import GridSystem from '@/components/layout/Grid';
import Link from 'next/link';
import FaqSection from '@/components/FAQ/FaqSection';

const FAQ = () => {
    const meta = {
        title: 'GLDT Swap FAQ',
        description: 'GLDT Swap FAQ Description',
    };

    return (
        <Layout meta={meta}>
            <FaqSection full={true} />
            {/* <GridSystem>
                <GridItem colStart={[1, 1, 1, 2]} colEnd={[12, 12, 12, 7]}>
                    <Box>
                        <Heading as="h1" variant="h2">
                            Frequently Asked Question
                        </Heading>
                    </Box>
                </GridItem>
                <GridItem colStart={[1, 1, 1, 4]} colEnd={[12, 12, 12, 12]}>
                    <Accordion allowToggle>
                        {Q.map((e, i) => (
                            <Question key={i} q={e.q} r={e.r} />
                        ))}
                    </Accordion>
                </GridItem>
            </GridSystem> */}
        </Layout>
    );
};

export default FAQ;
