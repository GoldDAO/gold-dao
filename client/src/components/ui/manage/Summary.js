import React, { useEffect } from 'react';
import {
    Card,
    Accordion,
    AccordionButton,
    AccordionPanel,
    AccordionItem,
    Text,
    CardBody,
    CardHeader,
} from '@chakra-ui/react';
import { useNft } from '@/query/hooks/useNFTs';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';

const Summary = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);

    return (
        <Card>
            <Overview nfts={nfts} isLoading={isLoading} />
            <Mynfts nfts={nfts} />
            <MyTransactions />
        </Card>
    );
};

export default Summary;

const Overview = ({ nfts, isLoading }) => {
    return <Card></Card>;
};

const Mynfts = ({ nfts, isLoading }) => {
    const weights = [1, 10, 100, 1000];
    useEffect(() => {
        console.log('nfts', nfts);
    }, [nfts]);
    return (
        <Card>
            <Accordion allowToggle>
                <AccordionItem>
                    <AccordionButton>
                        <Text>My Nfts</Text>
                    </AccordionButton>
                    <AccordionPanel>
                        {weights.map((weight, i) => (
                            <Card key={i}>
                                <CardHeader>GLDNFT {weight}g</CardHeader>
                                <CardBody>
                                    {nfts.map((e, i) => e.weight === weight && <>{e.name}</>)}
                                </CardBody>
                            </Card>
                        ))}
                    </AccordionPanel>
                </AccordionItem>
            </Accordion>
        </Card>
    );
};

const MyTransactions = () => {
    return <Card></Card>;
};
