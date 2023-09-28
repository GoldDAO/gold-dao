import React, { useEffect, useState } from 'react';
import {
    Card,
    Accordion,
    AccordionButton,
    AccordionPanel,
    AccordionItem,
    Text,
    CardBody,
    CardHeader,
    TableContainer,
    Table,
    Thead,
    Tr,
    Td,
    Tbody,
    Button,
    Box,
    Spinner,
} from '@chakra-ui/react';
import { useNft } from '@/query/hooks/useNFTs';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';
import { cancelSale } from '@/query/cancelSale';
import useSwapHistory from '@/query/hooks/useSwapHistory';

const Summary = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);

    return (
        <Card>
            <Overview nfts={nfts} isLoading={isLoading} />
            <Mynfts nfts={nfts} actors={actors} />
            <MyTransactions />
        </Card>
    );
};

export default Summary;

const Overview = ({ nfts, isLoading }) => {
    return <Card></Card>;
};

const Mynfts = ({ nfts, isLoading, actors }) => {
    const weights = [1, 10, 100, 1000];
    return (
        <Card>
            <CardHeader>
                <Text>My Nfts</Text>
            </CardHeader>
            <Accordion allowToggle>
                {weights.map((weight, i) => (
                    <AccordionItem>
                        <AccordionButton>GLDNFT {weight}g</AccordionButton>
                        <AccordionPanel>
                            <Card key={i}>
                                <CardBody>
                                    <TableContainer>
                                        <Table>
                                            <Thead>
                                                <Tr>
                                                    <Td>Token id</Td>
                                                    <Td>weight</Td>
                                                    <Td>Status</Td>
                                                </Tr>
                                            </Thead>
                                            <Tbody>
                                                {nfts.map((e, i) => {
                                                    if (e.weight === weight) {
                                                        return (
                                                            <Tr>
                                                                <Td>{e.name}</Td>
                                                                <Td>{e.weight}</Td>
                                                                <Td>
                                                                    <SaleStatus
                                                                        status={e.status}
                                                                        e={e}
                                                                    />
                                                                </Td>
                                                            </Tr>
                                                        );
                                                    }
                                                })}
                                            </Tbody>
                                        </Table>
                                    </TableContainer>
                                </CardBody>
                            </Card>
                        </AccordionPanel>
                    </AccordionItem>
                ))}
            </Accordion>
        </Card>
    );
};

const SaleStatus = ({ status, e }) => {
    const actors = useAllCanisters();
    const [saleStatus, setSaleStatus] = useState();
    const [isLoading, setIsloading] = useState(false);

    useEffect(() => {
        setSaleStatus(status ? true : false);
    }, []);

    const handleCancelSale = async (name, weight, actors) => {
        setIsloading(true);
        const res = await cancelSale(name, weight, actors);
        if (res[0].ok) {
            setSaleStatus(false);
            setIsloading(false);
        }
    };

    return saleStatus ? (
        <Button onClick={() => handleCancelSale(e.name, e.weight, actors)}>
            {isLoading && <Spinner size={'sm'} />}Cancel sale
        </Button>
    ) : (
        'Not on Sale'
    );
};

const MyTransactions = () => {
    const history = useSwapHistory();

    useEffect(() => {
        console.log('history', history);
    }, [history]);
    return (
        <Card>
            <CardHeader>
                <Text>My Transactions</Text>
            </CardHeader>
            <Accordion allowToggle>
                <AccordionItem>
                    <AccordionButton>Ongoing transactions</AccordionButton>
                    <AccordionPanel>
                        <Card>
                            <CardBody>
                                <TableContainer>
                                    <Table>
                                        <Thead>
                                            <Tr>
                                                <Td>Transaction id</Td>
                                                <Td>Token id</Td>
                                                <Td>timestamp</Td>
                                                <Td>GLDT</Td>
                                                <Td>Status</Td>
                                            </Tr>
                                        </Thead>
                                        <Tbody></Tbody>
                                    </Table>
                                </TableContainer>
                            </CardBody>
                        </Card>
                    </AccordionPanel>
                </AccordionItem>
                <AccordionItem>
                    <AccordionButton>Past transactions</AccordionButton>
                    <AccordionPanel>
                        <Card>
                            <CardBody>
                                <TableContainer>
                                    <Table>
                                        <Thead>
                                            <Tr>
                                                <Td>Transaction id</Td>
                                                <Td>Token id</Td>
                                                <Td>timestamp</Td>
                                                <Td>GLDT</Td>
                                                <Td>Status</Td>
                                            </Tr>
                                        </Thead>
                                        <Tbody></Tbody>
                                    </Table>
                                </TableContainer>
                            </CardBody>
                        </Card>
                    </AccordionPanel>
                </AccordionItem>
            </Accordion>
        </Card>
    );
};
