import { GridItem, Heading } from '@chakra-ui/react';
import React from 'react';

const TableTitle = ({ title }) => {
    return (
        <GridItem gridColumn={['1/13', '1/13', '1/2', '1/2', '1/2']}>
            <Heading
                fontWeight={300}
                as="h3"
                fontSize={'16px'}
                textAlign={'left'}
                w={'100%'}
                borderBottom="1px"
                borderBottomColor={'secondaryText'}
            >
                {title}
            </Heading>
        </GridItem>
    );
};

export default TableTitle;
