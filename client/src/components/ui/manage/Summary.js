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
    HStack,
} from '@chakra-ui/react';
import { useNft } from '@/query/hooks/useNFTs';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';
import { cancelSale } from '@/query/cancelSale';
import useSwapHistory from '@/query/hooks/useSwapHistory';

const Summary = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);

    return (
        <Card
            gridColumn={'3/11'}
            p={[2, 2, 2, 4]}
            shadow={['md', 'lg']}
            bg="bg"
            display="grid"
            gridTemplateRows={'repeat(1, 1fr)'}
            gap="3"
            borderRadius={'2xl'}
        >
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
        <Card
            bg="white"
            borderRadius={'lg'}
            border="1px"
            borderColor="border"
            shadow={'none'}
            p={[2, 2, 3, 4, 6]}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <CardHeader pl={0} py={'10px'}>
                <Text>My Nfts</Text>
            </CardHeader>
            <Accordion allowToggle>
                <HStack
                    wrap={'wrap'}
                    borderColor={'border'}
                    borderTop={0}
                    gap="2"
                    display="grid"
                    gridTemplateColumns={'repeat(2,1fr)'}
                    borderEndEndRadius={'md'}
                    borderEndStartRadius={'md'}
                >
                    {weights.map((weight, i) => (
                        <Card key={i} shadow={'none'} alignSelf={'flex-start'}>
                            <AccordionItem border={0}>
                                <AccordionButton
                                    shadow={'none'}
                                    h="60px"
                                    bg="bg"
                                    border="1px"
                                    borderColor="border"
                                    borderStartEndRadius={'md'}
                                    borderStartStartRadius={'md'}
                                >
                                    GLDNFT {weight}g
                                </AccordionButton>
                                <AccordionPanel
                                    shadow={'none'}
                                    bg="bg"
                                    borderEndEndRadius={'md'}
                                    borderEndStartRadius={'md'}
                                    border={'1px'}
                                    borderColor={'border'}
                                    borderTop={0}
                                >
                                    <Card key={i} shadow={'none'}>
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
                        </Card>
                    ))}
                </HStack>
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
        <Card
            bg="white"
            borderRadius={'lg'}
            border="1px"
            borderColor="border"
            shadow={'none'}
            p={[2, 2, 3, 4, 6]}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <CardHeader pl={0} py={'10px'}>
                <Text>My Transactions</Text>
            </CardHeader>
            <Accordion allowToggle>
                <HStack wrap={'wrap'}>
                    <AccordionItem w="100%" border={0}>
                        <AccordionButton
                            h="60px"
                            bg="bg"
                            border="1px"
                            borderColor="border"
                            borderStartEndRadius={'md'}
                            borderStartStartRadius={'md'}
                        >
                            Ongoing transactions
                        </AccordionButton>
                        <AccordionPanel
                            bg="bg"
                            borderEndEndRadius={'md'}
                            borderEndStartRadius={'md'}
                            border={'1px'}
                            borderColor={'border'}
                            borderTop={0}
                        >
                            <Card shadow={'none'}>
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
                    <AccordionItem w="100%" border={0}>
                        <AccordionButton
                            h="60px"
                            bg="bg"
                            border="1px"
                            borderColor="border"
                            borderStartEndRadius={'md'}
                            borderStartStartRadius={'md'}
                        >
                            Past transactions
                        </AccordionButton>
                        <AccordionPanel
                            bg="bg"
                            borderEndEndRadius={'md'}
                            borderEndStartRadius={'md'}
                            border={'1px'}
                            borderColor={'border'}
                            borderTop={0}
                        >
                            <Card shadow={'none'}>
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
                </HStack>
            </Accordion>
        </Card>
    );
};
