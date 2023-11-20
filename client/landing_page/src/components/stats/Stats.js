import {
    Grid,
    GridItem,
    LinkBox,
    Stack,
    Stat,
    StatGroup,
    StatLabel,
    StatNumber,
} from '@chakra-ui/react';
import useTotalSupply from '@utils/hooks/useTotalSupply';
import Link from 'next/link';
import React from 'react';

const Stats = () => {
    const totalSupply = useTotalSupply();
    const stats = [
        {
            label: 'USD Market Cap ',
            value: totalSupply.gldt
                ? `${(new Number(totalSupply.gldt) * 100).toLocaleString('en-US')} $`
                : '-',
        },
        {
            label: 'Gold Bars',
            value: '700',
        },
        {
            label: 'Gold Kilograms',
            value: totalSupply.g
                ? `${Math.round(totalSupply.g / 100).toLocaleString('en-US')} g`
                : '-',
        },
    ];
    return (
        <StatGroup
            display={'block'}
            w={'100%'}
            pt={['20px', '20px', '20px', 0]}
            mt={['30px', '40px', '40px', '80px']}
            justifyContent={'flex-start'}
        >
            {stats.map((e, i) => (
                <Stat
                    key={i}
                    w={['100%', '100%', 'fit-content']}
                    display={['block', 'block', 'inline-block']}
                    mt={['20px', '20px', 0]}
                    mr={[0, 0, '40px', '80px']}
                    mx={['auto', 'auto']}
                >
                    <StatLabel fontWeight={'bold'} m={0} textAlign={['center', 'center', 'left']}>
                        {e.label}
                    </StatLabel>
                    <StatNumber
                        textAlign={['center', 'center', 'left']}
                        fontWeight={300}
                        m={0}
                        fontSize={['24px', '24px', '28px', '36px']}
                    >
                        {e.value}
                    </StatNumber>
                </Stat>
            ))}
        </StatGroup>
    );
};

export default Stats;
