import { LinkBox, Stack, Stat, StatGroup, StatLabel, StatNumber } from '@chakra-ui/react';
import useTotalSupply from '@utils/hooks/useTotalSupply';
import Link from 'next/link';
import React from 'react';

const Stats = () => {
    const totalSupply = useTotalSupply();
    const stats = [
        {
            label: 'USD Market Cap ',
            value: `${(new Number(totalSupply.gldt) * 100).toLocaleString('en-US')} $`,
        },
        {
            label: 'Gold Bars',
            value: '700',
        },
        {
            label: 'Gold Kilograms',
            value: `${(totalSupply.g / 100).toLocaleString('en-US')} g`,
        },
    ];
    return (
        <StatGroup
            w={'100%'}
            justifyContent={['space-between', 'flex-start', 'flex-start', 'flex-start', 'flex-end']}
            pt={['20px', '20px', '20px', 0]}
        >
            <Stack
                direction={['row', 'row', 'row', 'column']}
                flexWrap={'wrap'}
                justifyContent={[
                    'space-between',
                    'flex-start',
                    'flex-start',
                    'flex-start',
                    'flex-end',
                ]}
                spacing={['20px']}
                w="100%"
            >
                {stats.map((e, i) => (
                    <Stat key={i} display={'flex'} w="33%" minW={'160px'}>
                        <StatLabel fontWeight={'bold'} m={0}>
                            {e.label}
                        </StatLabel>
                        <StatNumber fontWeight={300} m={0}>
                            {e.value}
                        </StatNumber>
                    </Stat>
                ))}
            </Stack>
        </StatGroup>
    );
};

export default Stats;
