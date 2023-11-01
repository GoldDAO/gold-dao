import { Box, Card, HStack, Heading, Stack, Text } from '@chakra-ui/react';
import React from 'react';
import { useNft } from '@utils/hooks/useNFTs';
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { useTotalSupply } from '@utils/hooks/useTotalSupply';
import TokenSign from '@ui/gldt/TokenSign';
import { cardPadding } from '@ui/theme';

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
        <Box gridColumn={['1/13', '1/13', '3/11', '3/11']}>
            <Heading as="h1" fontWeight={300}>
                Transparency
            </Heading>
            <Card
                mt="20px"
                gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
                p={cardPadding.xl}
                position={'relative'}
                bg="bg"
                mx={['10px', '20px', 0, 0, 0]}
                display="grid"
                justifyContent={'center'}
                gridTemplateRows={'repeat(1, 1fr)'}
                gridTemplateColumns={'repeat(1, 1fr)'}
                gap="3"
                borderRadius={'2xl'}
            >
                <Text>GLDT</Text>
                <HStack>
                    <Card w={'50%'}>
                        <HStack>
                            <Text>{totalSupply.gldt}</Text>
                            <TokenSign />
                        </HStack>
                    </Card>
                    <Card w={'50%'}>
                        <HStack>
                            <Text>{totalWeightSwapped} g</Text>
                            <TokenSign />
                        </HStack>
                    </Card>
                </HStack>
            </Card>
            <Card
                mt="20px"
                gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
                p={cardPadding.xl}
                position={'relative'}
                bg="bg"
                mx={['10px', '20px', 0, 0, 0]}
                display="grid"
                justifyContent={'center'}
                gridTemplateRows={'repeat(1, 1fr)'}
                gridTemplateColumns={'repeat(1, 1fr)'}
                gap="3"
                borderRadius={'2xl'}
            >
                <Stack wrap={'wrap'}>
                    {arr.map((e, i) => (
                        <Card key={i}>
                            <Text>{w[i]}g GLD Nfts</Text>
                            <Text>{e}g</Text>
                        </Card>
                    ))}
                </Stack>
            </Card>
        </Box>
    );
};

export default TransparencyContent;
