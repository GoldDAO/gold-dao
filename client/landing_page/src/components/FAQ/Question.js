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
    const mediumFontSize = ['24px', '26px', '28px', '32px', '32px'];
    const TextSize = ['18px', '18px', '20px', '24px', '28px'];
    return (
        <AccordionItem
            w={'100%'}
            borderTopColor={'black'}
            borderBottomColor={'black'}
            textAlign={'left'}
        >
            <AccordionButton w={'100%'} justifyContent={'space-between'} textAlign={'left'}>
                <Text fontSize={mediumFontSize} py="20px" m={0}>
                    {q}
                </Text>
                <AccordionIcon h={'33px'} w={'33px'} />
            </AccordionButton>
            <AccordionPanel fontSize={TextSize}>{r}</AccordionPanel>
        </AccordionItem>
    );
};

export default Question;
