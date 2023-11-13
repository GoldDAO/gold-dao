import { GridItem, Heading } from '@chakra-ui/react';
import PrincipalFormat from '@ui/principal/Principal';
import React from 'react';

const Title = ({ title, subTitle }) => {
    return (
        <GridItem
            gridColumn={['1/12', '1/12', '2/12, 2/6']}
            py={['0px', '0px', '40px']}
            fontSize={['26px', '26px', '42px']}
        >
            <Heading as="h1" variant={'h1'} fontSize={'inherit'} p="0" m="0" lineHeight={1}>
                {title}
            </Heading>
            <Heading as="h2" variant={'h2'} fontSize={'inherit'} p="0" m="0" lineHeight={1}>
                {subTitle}
            </Heading>
        </GridItem>
    );
};

export default Title;
