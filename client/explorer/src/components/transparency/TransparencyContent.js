import { Box, Card, GridItem, HStack, Heading, Stack, Text } from '@chakra-ui/react';
import React from 'react';
import { useNft } from '@utils/hooks/useNFTs';
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { useTotalSupply } from '@utils/hooks/useTotalSupply';
import TokenSign from '@ui/gldt/TokenSign';
import { cardPadding } from '@ui/theme';
import Layout from '../layout/Layout';
import GridSystem from '@ui/layout/GridSystem';

const TransparencyContent = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors, 'm45be-jaaaa-aaaak-qcgnq-cai');
    const totalSupply = useTotalSupply();

    const totalWeightSwapped = (totalSupply.gldt / 100).toFixed(2);

    const getTotalWeight = (nfts, w) =>
        nfts.reduce((ac, e) => {
            if (e.weight === w) {
                return ac + e.weight;
            }
            return ac;
        }, 0);

    const arr = [
        getTotalWeight(nfts, 1),
        getTotalWeight(nfts, 10),
        getTotalWeight(nfts, 100),
        getTotalWeight(nfts, 1000),
    ];

    const w = [1, 10, 100, 1000];
    return (
        <GridSystem>
            <GridItem gridColumn={['1/12', '1/12', '2/12']} py="40px">
                <Heading as="h1" variant={'h1'}>
                    GLDT
                </Heading>
                <Heading as="h2" variant={'h2'}>
                    Transparency
                </Heading>
            </GridItem>
            <GridItem colSpan={'8'} colStart={[1, 1, 2]}>
                <Text fontSize={'16px'}>Total Supply</Text>
                <HStack fontSize={'34px'}>
                    <Text fontSize={'inherit'}>{totalSupply.gldt}</Text>
                    <TokenSign />
                </HStack>
            </GridItem>
            <GridItem colSpan={'8'} colStart={[1, 1, 2]}>
                <Text fontSize={'16px'}>Total Swapped</Text>
                <HStack fontSize={'34px'}>
                    <Text fontSize={'inherit'}>{totalWeightSwapped} g</Text>
                </HStack>
            </GridItem>
            {arr.map((e, i) => (
                <GridItem colSpan={'8'} colStart={[1, 1, 2]} key={i} fontSize={'34px'}>
                    <Text fontSize={'16px'}>{w[i]}g GLD Nfts</Text>
                    <Text fontSize={'inherit'}>{e}g</Text>
                </GridItem>
            ))}
        </GridSystem>
    );
};

export default TransparencyContent;
