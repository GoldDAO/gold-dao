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
            borderBottomColor={'transparent'}
            textAlign={'left'}
        >
            <AccordionButton
                pl={0}
                w={'100%'}
                justifyContent={'space-between'}
                textAlign={'left'}
                py="1em"
            >
                <Text w={'90%'} m={0} fontWeight={500}>
                    {q}
                </Text>
                <AccordionIcon />
            </AccordionButton>
            <AccordionPanel pb="1em" pl={0} width={['90%', '90%', '90%', '75%', '75%']}>
                <Text>{r}</Text>
            </AccordionPanel>
        </AccordionItem>
    );
};

export default Question;
