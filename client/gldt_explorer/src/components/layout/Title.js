import { GridItem, Heading } from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';
import React from 'react';

const Title = ({ title, subTitle, cp }) => {
    return (
        <GridItem
            gridColumn={['1/12', '1/12', '2/12, 2/6']}
            fontSize={['32px', '32px', '46px']}
            pb="20px"
        >
            {title && (
                <Heading as="h1" variant={'h1'} fontSize={'inherit'} p="0" m="0" lineHeight={1}>
                    {title}
                </Heading>
            )}
            <Heading as="h2" variant={'h2'} fontSize={'inherit'} p="0" m="0" lineHeight={1}>
                {subTitle}
                {cp && <CopyPrincipal text={subTitle} />}
            </Heading>
        </GridItem>
    );
};

export default Title;
