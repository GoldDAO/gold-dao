import { Box, Card, GridItem, HStack, Heading, Stack, Text } from '@chakra-ui/react';
import React from 'react';
import { useNft } from '@utils/hooks/useNFTs';
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { useTotalSupply } from '@utils/hooks/useTotalSupply';
import TokenSign from '@ui/gldt/TokenSign';
import { cardPadding } from '@ui/theme';
import Layout from '../layout/Layout';
import GridSystem from '@ui/layout/GridSystem';
import Title from '../layout/Title';

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
            <Title title={'GLDT'} subTitle={'Transparency'} />
            <GridItem colSpan={['12', '12', '12']}>
                <Text fontSize={'16px'} width={['100%', '100%', '50%']}>
                    GLDT are minted at a ratio of 100 GLDT per gram of GLD NFT. GLDT is about
                    transparency and let's everyone verify themselves that the ratio of GLDT to GLD
                    NFT in the swap contract is valid.
                </Text>
            </GridItem>
            <GridItem colSpan={['12', '12', '6', '3']} py={['10px', '10px', '20px']}>
                <Text fontSize={'14px'} fontWeight={500}>
                    Total Supply
                </Text>
                <HStack fontSize={'34px'} fontWeight={300}>
                    <Text fontWeight={300} fontSize={'inherit'} fontFamily={'inter'}>
                        {totalSupply.gldt}
                    </Text>
                    <TokenSign />
                </HStack>
            </GridItem>
            <GridItem colSpan={['12', '12', '6', '3']} py={['10px', '10px', '20px']}>
                <Text fontSize={'14px'} fontWeight={500}>
                    Total Swapped
                </Text>
                <HStack fontSize={'34px'}>
                    <Text fontSize={'inherit'} fontWeight={'200'} fontFamily={'inter'}>
                        {totalWeightSwapped} g
                    </Text>
                </HStack>
            </GridItem>
            {arr.map((e, i) => (
                <GridItem
                    colSpan={['12', '12', '6', '3']}
                    key={i}
                    fontSize={'34px'}
                    py={['10px', '10px', '20px']}
                >
                    <Text fontSize={'14px'} fontWeight={500}>
                        {w[i]}g GLD Nfts
                    </Text>
                    <Text fontSize={'inherit'} fontFamily={'inter'} fontWeight={'200'}>
                        {e}g
                    </Text>
                </GridItem>
            ))}
        </GridSystem>
    );
};

export default TransparencyContent;
