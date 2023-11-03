import { Stack, Stat, StatGroup, StatLabel, StatNumber } from '@chakra-ui/react';
import React from 'react';

const Stats = ({ stats }) => {
    return (
        <StatGroup w={'100%'} justifyContent={'flex-end'}>
            <Stack direction={['column']} spacing={['20px']}>
                {stats.map((e, i) => (
                    <Stat key={i} display={'flex'}>
                        <StatLabel fontWeight={'bold'} m={0}>
                            {e.label}
                        </StatLabel>
                        <StatNumber fontWeight={400} m={0}>
                            {e.value}
                        </StatNumber>
                    </Stat>
                ))}
            </Stack>
        </StatGroup>
    );
};

export default Stats;
