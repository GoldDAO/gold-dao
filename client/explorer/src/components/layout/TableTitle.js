import { GridItem, Heading } from '@chakra-ui/react';
import React from 'react';

const TableTitle = ({ title }) => {
    return (
        <GridItem
            gridColumn={['1/13', '1/13', '1/12', '1/12', '1/12']}
            pt={['40px', '40px', 0]}
            pb={['20px', '20px', 0]}
        >
            <Heading
                fontWeight={300}
                as="h4"
                fontSize={'16px'}
                textAlign={'left'}
                w={'100%'}
                borderBottomColor={'secondaryText'}
                color={'secondaryText'}
            >
                {title}
            </Heading>
        </GridItem>
    );
};

export default TableTitle;
