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
        <AccordionItem
            w={'100%'}
            borderTopColor={'lightGold'}
            borderBottomColor={'lightGold'}
            textAlign={'left'}
        >
            <AccordionButton
                pl={0}
                w={'100%'}
                justifyContent={'space-between'}
                textAlign={'left'}
                py="1em"
            >
                <Text m={0}>{q}</Text>
                <AccordionIcon />
            </AccordionButton>
            <AccordionPanel py="1em" width={['100%', '100%', '100%', '100%', '75%']}>
                {r}
            </AccordionPanel>
        </AccordionItem>
    );
};

export default Question;
