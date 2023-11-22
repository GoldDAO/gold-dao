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
    Stack,
    HStack,
    Skeleton,
    AccordionIcon,
    VStack,
    Flex,
} from '@chakra-ui/react';
import { useNft } from '@utils/hooks/gldnfts/useNFTs';
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { cancelSale } from '@utils/queries/cancelSale';
import useSwapHistory, { useMaxEntry } from '@utils/hooks/gldtCore/useSwapHistory';
import { useConnect } from '@connect2ic/react';
import useOngoingSwaps from '@utils/hooks/gldtCore/useOngoingSwap';
import TokenSign from '@ui/gldt/TokenSign';
import { ArrowBackIcon, ArrowForwardIcon, ChevronDownIcon, SmallCloseIcon } from '@chakra-ui/icons';
import NFTIcon from '/public/images/sell.svg';
import weightIcon from '/public/images/scale.svg';
import swappedIcon from '/public/images/send_money.svg';
import Image from 'next/image';
import Timestamp from '@ui/tooltip/timeStamp';
import { cardPadding } from '@ui/theme';

const Summary = () => {
    const { isConnected } = useConnect();
    return (
        <Card
            mt="20px"
            gridColumn={['1/12', '1/12', '2/12']}
            p={cardPadding.xl}
            mx={['10px', '20px', 0, 0, 0]}
            display="grid"
            gridTemplateRows={'repeat(1, 1fr)'}
            gap="3"
            borderRadius={'2xl'}
            shadow={'none'}
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
    const history = useSwapHistory(0, 10);

    const totalweight = nfts.reduce((ac, e) => {
        return ac + e.weight;
    }, 0);
    const nftsN = nfts.length;

    const totalSwap = history?.history?.Ok?.data[0]?.reduce((ac, e) => {
        return ac + parseInt(e.num_tokens.value) / 100000000;
    }, 0);

    const BigN = ({ children }) => {
        return (
            <Text fontWeight={200} fontSize={['1.2em', '1.2em', '1.6em']}>
                {children}
            </Text>
        );
    };
    return (
        <Card shadow={'none'} borderRadius={'lg'}>
            <Stack
                spacing={{ base: 0, md: '15px' }}
                direction={['column', 'column', 'row']}
                style={{
                    opacity: isLoading ? 0.5 : 1,
                }}
            >
                <Card shadow={'none'}>
                    <CardBody py={['7px', '10px', '20px']}>
                        <HStack direction={['column', 'column', 'row']} alignItems={'flex-start'}>
                            <Image
                                alt="NFT icon"
                                src={NFTIcon}
                                width={'40px'}
                                style={{
                                    opacity: 0.5,
                                    paddingTop: '7px',
                                }}
                            />
                            <Stack direction={['row', 'row', 'column']}>
                                <Box>
                                    {!isLoading ? (
                                        <BigN>{nfts.length}</BigN>
                                    ) : (
                                        <Spinner size={'sm'} color="secondaryText" />
                                    )}
                                    <Text fontSize={'14px'}>Total number of NFTs</Text>
                                </Box>
                            </Stack>
                        </HStack>
                    </CardBody>
                </Card>
                <Card shadow={'none'}>
                    <CardBody py={['7px', '10px', '20px']}>
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
                                <Text fontSize={'14px'}>Total NFTs weight</Text>
                            </Box>
                        </HStack>
                    </CardBody>
                </Card>
                <Card shadow={'none'}>
                    <CardBody py={['7px', '10px', '20px']}>
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
                                        <BigN>{parseInt(totalSwap) || 0}</BigN>
                                        <TokenSign />
                                    </HStack>
                                ) : (
                                    <Spinner size={'sm'} color="secondaryText" />
                                )}
                                <Text fontSize={'14px'}>Total GLDT swapped</Text>
                            </Box>
                        </HStack>
                    </CardBody>
                </Card>
            </Stack>
        </Card>
    );
};

const Mynfts = ({ connected }) => {
    const weights = [1, 10, 100, 1000];
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors);
    const [nftCounter, setNftCounter] = useState([0, 0, 0, 0]);
    const weightsToIndex = {
        1: 0,
        10: 1,
        100: 2,
        1000: 3,
    };
    useEffect(() => {
        for (let i = 0; i <= nfts.length - 1; i++) {
            setNftCounter((prev) => {
                const weightIndex = weightsToIndex[nfts[i].weight];
                const updatedCounter = [...prev];
                updatedCounter[weightIndex] += 1;
                return updatedCounter;
            });
        }
    }, [nfts]);

    const sortNfts = (nfts) => {
        const nftsByW = {
            1: [],
            10: [],
            100: [],
            1000: [],
        };

        nfts.map((e) => {
            nftsByW[e.weight].push(e);
        });
        return nftsByW;
    };

    return (
        <Card
            bg="white"
            borderRadius={0}
            borderTop="1px"
            borderColor="border"
            shadow={'none'}
            p={[2, 2, 3, 4, 6]}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <CardHeader pl={0} py={{ base: '2px', md: '10px' }}>
                <Text fontSize={'16px'}>My Nfts</Text>
            </CardHeader>
            <Accordion allowToggle>
                <HStack
                    wrap={'wrap'}
                    borderColor={'border'}
                    borderTop={0}
                    gap="2"
                    display="grid"
                    gridTemplateColumns={['repeat(1,1fr)', 'repeat(1,1fr)', 'repeat(2,1fr)']}
                    borderEndEndRadius={'md'}
                    borderEndStartRadius={'md'}
                    direction={['column', 'column', 'column', 'column', 'column']}
                >
                    {Object.keys(sortNfts(nfts)).map((weight, i) => {
                        return (
                            <Card key={i} shadow={'none'} alignSelf={'flex-start'}>
                                <AccordionItem
                                    border={0}
                                    isDisabled={
                                        isLoading || sortNfts(nfts)[weight].length > 0
                                            ? false
                                            : true
                                    }
                                >
                                    <AccordionButton
                                        shadow={'none'}
                                        h="60px"
                                        bg="bg"
                                        fontSize={'18px'}
                                        border="1px"
                                        borderColor="border"
                                        borderRadius={'md'}
                                        display={'flex'}
                                    >
                                        <HStack
                                            width={'100%'}
                                            justifyContent={'space-between'}
                                            borderStartStartRadius={'md'}
                                        >
                                            <Text fontSize={'16px'}>
                                                {sortNfts(nfts)[weight].length > 0
                                                    ? `GLD NFT ${weight}g`
                                                    : `NO ${weight}g GLD NFT`}
                                            </Text>

                                            {isLoading ? <Spinner size="md" /> : <AccordionIcon />}
                                        </HStack>
                                    </AccordionButton>
                                    <AccordionPanel
                                        shadow={'none'}
                                        bg="bg"
                                        borderRadius={'md'}
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
                                                                    sortNfts(nfts)[weight].map(
                                                                        (e, j) => {
                                                                            return (
                                                                                <Tr key={j}>
                                                                                    <Td>
                                                                                        {e.name}
                                                                                    </Td>
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
                                                                        },
                                                                    )}
                                                            </Tbody>
                                                        </Table>
                                                    </TableContainer>
                                                </CardBody>
                                            </Card>
                                        )}
                                    </AccordionPanel>
                                </AccordionItem>
                            </Card>
                        );
                    })}
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
            fontSize={'14px'}
            w={'fit-content'}
        >
            Not on Sale
        </Text>
    );
};

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total }) => {
    total = total ? Number(total) : 0;
    return (
        <VStack p="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text fontSize={'16px'}>Page {currentHistoryPage + 1}</Text>
                <Text fontSize={'16px'}>{total} entries</Text>
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
    const history = useSwapHistory(currentHistoryPage, 10);
    const ongoing = useOngoingSwaps(true, currentOngoingPage);

    return (
        <Card
            bg="white"
            borderRadius={0}
            borderTop="1px"
            borderColor="border"
            shadow={'none'}
            p={[2, 2, 3, 4, 6]}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <CardHeader pl={0} py={{ base: '2px', md: '10px' }}>
                <Text fontSize={'16px'}>My Transactions</Text>
            </CardHeader>
            <Accordion allowToggle>
                <HStack wrap={'wrap'}>
                    <AccordionItem w="100%" border={0} isDisabled={ongoing.isLoading}>
                        <AccordionButton
                            h="60px"
                            bg="bg"
                            border="1px"
                            borderColor="border"
                            borderRadius={'md'}
                            display={'flex'}
                            justifyContent={'space-between'}
                        >
                            Ongoing transactions
                            {ongoing.isLoading ? <Spinner size="md" /> : <AccordionIcon />}
                        </AccordionButton>
                        <AccordionPanel
                            bg="bg"
                            borderRadius={'md'}
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
                                                {ongoing?.ongoing?.Ok.data[0].length < 1 && (
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
                            borderRadius={'md'}
                            display={'flex'}
                            justifyContent={'space-between'}
                        >
                            Past transactions
                            {history.isLoading ? <Spinner size="md" /> : <AccordionIcon />}
                        </AccordionButton>
                        <AccordionPanel bg="bg" border={'1px'} borderColor={'border'} borderTop={0}>
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
                                                {history?.history?.Ok.data[0]?.length === 0 && (
                                                    <Tr>
                                                        <Td>You have no ongoing swaps</Td>
                                                    </Tr>
                                                )}
                                                {history?.history?.Ok.data[0]?.map((e, i) => {
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
