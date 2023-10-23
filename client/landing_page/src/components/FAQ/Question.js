import {
    Accordion,
    AccordionButton,
    AccordionIcon,
    AccordionItem,
    AccordionPanel,
    Text,
} from '@chakra-ui/react';
import React from 'react';

const Question = ({ q, r }) => {
    return (
        <AccordionItem w={'100%'} borderTopColor={'black'}>
            <AccordionButton w={'100%'} justifyContent={'space-between'}>
                <Text fontSize={'32px'} py="20px" m={0}>
                    {q}
                </Text>
                <AccordionIcon h={'33px'} w={'33px'} />
            </AccordionButton>
            <AccordionPanel fontSize={'32px'}>{r}</AccordionPanel>
        </AccordionItem>
    );
};

export default Question;
