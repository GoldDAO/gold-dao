import React from 'react';
import useTotalSupply from '@utils/hooks/useTotalSupply';
import { Stat, StatLabel, StatNumber } from '@chakra-ui/react';

const Supply = () => {
    const supply = useTotalSupply();
    const mediumFontSize = ['24px', '26px', '28px', '32px', '36px'];
    const TextSize = ['18px', '18px', '20px', '22px', '24px'];

    const stats = [
        {
            label: 'USD Market Cap ',
            number: `${supply.gldt} $`,
        },
        {
            label: 'Gold Bars ',
            number: '700 bars',
        },
        {
            label: 'Gold Kilograms',
            number: `${supply.g / 1000} g`,
        },
    ];
    return stats.map((e, i) => (
        <Stat key={i} display={'flex'}>
            <StatLabel
                fontSize={TextSize}
                fontWeight={'bold'}
                pb={['5px', '5px', '20px', '20px', '25px']}
                m={0}
            >
                {e.label}
            </StatLabel>
            <StatNumber
                fontSize={mediumFontSize}
                fontWeight={400}
                m={0}
                pb={['15px', '15px', '25px', '0', '0']}
            >
                {e.number}
            </StatNumber>
        </Stat>
    ));
};

export default Supply;
