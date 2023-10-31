import Question from '@/components/FAQ/Question';
import Layout from '@/components/layout/Layout';
import { Accordion, Box, Heading } from '@chakra-ui/react';
import React from 'react';
import Q from './../src/components/FAQ/content.json';

const FAQ = () => {
    const meta = {
        title: 'GLDT Swap FAQ',
        description: 'GLDT Swap FAQ Description',
    };
    const margins = ['20px', '30px', '60px', '140px', '180px'];
    const titleFontSize = ['40px', '60px', '60px', '80px', '96px'];
    const verticalSpacing = ['60px', '80px', '80px', '140px', '180px'];
    return (
        <Layout meta={meta}>
            <Box m="0 auto" px={margins}>
                <Heading as="h1" fontSize={titleFontSize} my={verticalSpacing}>
                    Frequently Asked Questions
                </Heading>
                <Accordion allowToggle>
                    {Q.map((e, i) => (
                        <Question key={i} q={e.q} r={e.r} />
                    ))}
                </Accordion>
            </Box>
        </Layout>
    );
};

export default FAQ;
