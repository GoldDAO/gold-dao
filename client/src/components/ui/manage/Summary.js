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
    Skeleton,
    AccordionIcon,
    VStack,
    Tfoot,
    Flex,
} from '@chakra-ui/react';
import { useNft } from '@/query/hooks/useNFTs';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';
import { cancelSale } from '@/query/cancelSale';
import useSwapHistory from '@/query/hooks/useSwapHistory';
import { useConnect } from '@connect2ic/react';
import useOngoingSwaps from '@/query/hooks/useOngoingSwap';
import TokenSign from '../gldt/TokenSign';
import { ArrowBackIcon, ArrowForwardIcon, ChevronDownIcon, SmallCloseIcon } from '@chakra-ui/icons';
import NFTIcon from '/public/images/sell.svg';
import weightIcon from '/public/images/scale.svg';
import swappedIcon from '/public/images/send_money.svg';
import Image from 'next/image';
import Timestamp from '../tooltip/timeStamp';
const Summary = () => {
    const { isConnected } = useConnect();
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
            {isConnected ? <Overview connected={isConnected} /> : <Skeleton h={'100px'} />}
            {isConnected ? <Mynfts connected={isConnected} /> : <Skeleton h={'200px'} />}
            {isConnected ? <MyTransactions connected={isConnected} /> : <Skeleton h={'150px'} />}
        </Card>
    );
};

export default Summary;

const Overview = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);
    const history = useSwapHistory();
    const totalweight = nfts.reduce((ac, e) => {
        return ac + e.weight;
    }, 0);
    const nftsN = nfts.length;

    const totalSwap = history.history?.Ok?.data[0].reduce((ac, e) => {
        return ac + parseInt(e.num_tokens.value) / 100000000;
    }, 0);

    const BigN = ({ children }) => {
        return (
            <Text fontWeight={200} fontSize={'1.6em'}>
                {children}
            </Text>
        );
    };
    return (
        <Card shadow={'none'} border={'1px'} borderColor={'border'} borderRadius={'lg'}>
            <HStack
                spacing="40px"
                style={{
                    opacity: isLoading ? 0.5 : 1,
                }}
            >
                <Card shadow={'none'}>
                    <CardBody>
                        <HStack align="start">
                            <Image
                                alt="NFT icon"
                                src={NFTIcon}
                                width={'40px'}
                                style={{
                                    opacity: 0.5,
                                    paddingTop: '7px',
                                }}
                            />
                            <HStack>
                                <Box>
                                    {!isLoading ? (
                                        <BigN>{nfts.length}</BigN>
                                    ) : (
                                        <Spinner size={'sm'} color="secondaryText" />
                                    )}
                                    <Text>Total number of NFTs</Text>
                                </Box>
                            </HStack>
                        </HStack>
                    </CardBody>
                </Card>
                <Card shadow={'none'}>
                    <CardBody>
                        <HStack align="start">
                            <Image
                                alt="scale icon"
                                src={weightIcon}
                                width={'40px'}
                                style={{
                                    opacity: '.5',
                                    paddingTop: '7px',
                                }}
                            />
                            <Box>
                                {!isLoading ? (
                                    <BigN>{totalweight} g</BigN>
                                ) : (
                                    <Spinner size={'sm'} color="secondaryText" />
                                )}
                                <Text>Total NFTs weight</Text>
                            </Box>
                        </HStack>
                    </CardBody>
                </Card>
                <Card shadow={'none'}>
                    <CardBody>
                        <HStack align="start">
                            <Image
                                alt="swapped coins icon"
                                src={swappedIcon}
                                width={'40px'}
                                style={{
                                    paddingTop: '7px',
                                    opacity: '.5',
                                }}
                            />
                            <Box>
                                {!isLoading ? (
                                    <HStack>
                                        <BigN>{totalSwap}</BigN>
                                        <TokenSign />
                                    </HStack>
                                ) : (
                                    <Spinner size={'sm'} color="secondaryText" />
                                )}
                                <Text>Total GLDT swapped</Text>
                            </Box>
                        </HStack>
                    </CardBody>
                </Card>
            </HStack>
        </Card>
    );
};

const Mynfts = ({ connected }) => {
    const weights = [1, 10, 100, 1000];
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);
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
                            <AccordionItem border={0} isDisabled={isLoading}>
                                <AccordionButton
                                    shadow={'none'}
                                    h="60px"
                                    bg="bg"
                                    border="1px"
                                    borderColor="border"
                                    borderStartEndRadius={'md'}
                                    borderStartStartRadius={'md'}
                                    display={'flex'}
                                >
                                    <HStack
                                        width={'100%'}
                                        justifyContent={'space-between'}
                                        borderStartStartRadius={'md'}
                                    >
                                        <Text>GLD NFT {weight}g</Text>

                                        {isLoading ? <Spinner size="md" /> : <AccordionIcon />}
                                    </HStack>
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
                                    {connected && (
                                        <Card key={i} shadow={'none'}>
                                            <CardBody p={0}>
                                                <TableContainer>
                                                    <Table>
                                                        <Thead>
                                                            <Tr>
                                                                <Td>Token id</Td>
                                                                <Td>Status</Td>
                                                            </Tr>
                                                        </Thead>
                                                        <Tbody p={0}>
                                                            {!isLoading &&
                                                                nfts.map((e, i) => {
                                                                    if (e.weight === weight) {
                                                                        return (
                                                                            <Tr key={i}>
                                                                                <Td>{e.name}</Td>
                                                                                <Td>
                                                                                    <SaleStatus
                                                                                        status={
                                                                                            e.status
                                                                                        }
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
                                    )}
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
        <Button
            onClick={() => handleCancelSale(e.name, e.weight, actors)}
            borderRadius={'200px'}
            bg="white"
            size={'sm'}
            fontWeight={400}
            border={'1px'}
            borderColor={'black'}
        >
            {isLoading ? (
                <Spinner size={'sm'} />
            ) : (
                <SmallCloseIcon
                    border={'1px'}
                    borderColor={'black'}
                    borderRadius={'200px'}
                    mr="10px"
                />
            )}
            Cancel sale
        </Button>
    ) : (
        <Text
            border={'1px'}
            borderColor={'border'}
            color={'secondaryText'}
            borderRadius={'200px'}
            p={'3px 12px'}
            w={'fit-content'}
        >
            Not on Sale
        </Text>
    );
};

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total }) => {
    total = total ? total : 0;
    return (
        <VStack p="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text>Page {currentHistoryPage + 1}</Text>
                <Text>{total} entries</Text>
            </Flex>
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Button
                    bg="bg"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={currentHistoryPage < 1}
                    onClick={() => setCurrentHistoryPage((prev) => prev - 1)}
                >
                    <ArrowBackIcon />
                </Button>
                <Button
                    bg="bg"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={total / (currentHistoryPage + 1) < 10}
                    onClick={() => setCurrentHistoryPage((prev) => prev + 1)}
                >
                    <ArrowForwardIcon />
                </Button>
            </Flex>
        </VStack>
    );
};

const MyTransactions = () => {
    const [currentHistoryPage, setCurrentHistoryPage] = useState(0);
    const [currentOngoingPage, setCurrentOngoingPage] = useState(0);
    const history = useSwapHistory(currentHistoryPage);
    const ongoing = useOngoingSwaps(true, currentOngoingPage);

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
                    <AccordionItem w="100%" border={0} isDisabled={ongoing.isLoading}>
                        <AccordionButton
                            h="60px"
                            bg="bg"
                            border="1px"
                            borderColor="border"
                            borderStartEndRadius={'md'}
                            borderStartStartRadius={'md'}
                            display={'flex'}
                            justifyContent={'space-between'}
                        >
                            Ongoing transactions
                            {ongoing.isLoading ? <Spinner size="md" /> : <AccordionIcon />}
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
                                <CardBody p={0}>
                                    <TableContainer>
                                        <Table>
                                            <Thead>
                                                <Tr>
                                                    <Td>Type</Td>
                                                    <Td>Date</Td>
                                                    <Td>Token id</Td>
                                                    <Td>GLDT amount</Td>
                                                    <Td>Status</Td>
                                                </Tr>
                                            </Thead>
                                            <Tbody>
                                                {ongoing.ongoing?.Ok.data[0].length < 1 && (
                                                    <Tr>
                                                        <Td>You have no ongoing swaps</Td>
                                                        <Td></Td>
                                                        <Td></Td>
                                                        <Td></Td>
                                                        <Td></Td>
                                                    </Tr>
                                                )}
                                                {ongoing.ongoing?.Ok.data[0].map((e, i) => {
                                                    return (
                                                        <Tr key={i}>
                                                            <Td>{Object.keys(e.record_type)}</Td>
                                                            <Td>
                                                                <Timestamp
                                                                    timestamp={parseInt(
                                                                        e.timestamp,
                                                                    )}
                                                                />
                                                            </Td>
                                                            <Td>{e.nft_id}</Td>
                                                            <Td>
                                                                <HStack>
                                                                    <Text>
                                                                        {parseInt(
                                                                            e.num_tokens.value,
                                                                        ) / 100000000}
                                                                    </Text>
                                                                    <TokenSign />
                                                                </HStack>
                                                            </Td>
                                                            <Td>{Object.keys(e.status.status)}</Td>
                                                        </Tr>
                                                    );
                                                })}
                                            </Tbody>
                                        </Table>
                                        <Pagination
                                            total={ongoing.ongoing?.Ok?.total}
                                            currentHistoryPage={currentOngoingPage}
                                            setCurrentHistoryPage={setCurrentOngoingPage}
                                        />
                                    </TableContainer>
                                </CardBody>
                            </Card>
                        </AccordionPanel>
                    </AccordionItem>
                    <AccordionItem w="100%" border={0} isDisabled={history.isLoading}>
                        <AccordionButton
                            h="60px"
                            bg="bg"
                            border="1px"
                            borderColor="border"
                            borderStartEndRadius={'md'}
                            borderStartStartRadius={'md'}
                            display={'flex'}
                            justifyContent={'space-between'}
                        >
                            Past transactions
                            {history.isLoading ? <Spinner size="md" /> : <AccordionIcon />}
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
                                <CardBody p={0}>
                                    <TableContainer>
                                        <Table
                                            style={{
                                                opacity: history.isLoading ? 0.5 : 1,
                                            }}
                                        >
                                            <Thead>
                                                <Tr>
                                                    <Td>Type</Td>
                                                    <Td>Date</Td>
                                                    <Td>Token id</Td>
                                                    <Td>GLDT amount</Td>
                                                    <Td>Status</Td>
                                                </Tr>
                                            </Thead>
                                            <Tbody>
                                                {history.history?.Ok.data[0].length === 0 && (
                                                    <Tr>
                                                        <Td>You have no ongoing swaps</Td>
                                                    </Tr>
                                                )}
                                                {history.history?.Ok.data[0].map((e, i) => {
                                                    return (
                                                        <Tr key={i}>
                                                            <Td>{Object.keys(e.record_type)}</Td>
                                                            <Td>
                                                                <Timestamp
                                                                    timestamp={parseInt(
                                                                        e.timestamp,
                                                                    )}
                                                                />
                                                            </Td>
                                                            <Td>{e.nft_id}</Td>
                                                            <Td>
                                                                <HStack>
                                                                    <Text>
                                                                        {parseInt(
                                                                            e.num_tokens.value,
                                                                        ) / 100000000}
                                                                    </Text>
                                                                    <TokenSign />
                                                                </HStack>
                                                            </Td>
                                                            <Td>{Object.keys(e.status.status)}</Td>
                                                        </Tr>
                                                    );
                                                })}
                                            </Tbody>
                                        </Table>
                                        <Pagination
                                            total={history.history?.Ok?.total}
                                            currentHistoryPage={currentHistoryPage}
                                            setCurrentHistoryPage={setCurrentHistoryPage}
                                        />
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
