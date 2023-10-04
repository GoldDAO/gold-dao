import React, { useEffect, useState } from 'react';
import {
    Accordion,
    AccordionButton,
    AccordionItem,
    AccordionPanel,
    Box,
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    IconButton,
    Modal,
    ModalContent,
    ModalHeader,
    ModalOverlay,
    Spinner,
    Table,
    TableContainer,
    TagCloseButton,
    Tbody,
    Td,
    Tr,
    useDisclosure,
    Tooltip,
    Tag,
    TagLabel,
    TagRightIcon,
    CircularProgress,
    Skeleton,
    HStack,
    Text,
    Th,
    Stack,
    ModalBody,
    ModalFooter,
    CheckboxIcon,
    AccordionIcon,
} from '@chakra-ui/react';
import {
    addCartItemAtom,
    cartAtom,
    getCartAtom,
    getTotalCartWeightAtom,
    removeAllItemsInCartAtom,
    removeCartItemByIdAtom,
} from '@/atoms/cart';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';
import { useNft } from '@/query/hooks/useNFTs';
import { useAtom } from 'jotai';
import { useConnect } from '@connect2ic/react';
import { sendBatchOffer } from '@/query/sendBatchOffer';
import Link from 'next/link';
import TokenSign from '../gldt/TokenSign';
import Grid from '@/components/layout/Grid';
import { CheckCircleIcon, ChevronDownIcon, InfoIcon, WarningIcon } from '@chakra-ui/icons';
import Image from 'next/image';
import Arrow from '/public/images/arrow.svg';
import { gldNftCanisters } from '@/services/agents';
import { cardPadding } from '@/theme/theme';

const SwapInterface = () => {
    const { isConnected } = useConnect();
    return (
        <Card
            mt="20px"
            gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
            p={cardPadding.xl}
            position={'relative'}
            shadow={['md', 'lg']}
            bg="bg"
            mx={['10px', '20px', 0, 0, 0]}
            display="grid"
            justifyContent={'center'}
            gridTemplateRows={'repeat(1, 1fr)'}
            gridTemplateColumns={'repeat(1, 1fr)'}
            gap="3"
            borderRadius={'2xl'}
        >
            <Input isConnected={isConnected} />
            <Output isConnected={isConnected} />
            <SwapButton isConnected={isConnected} />
        </Card>
    );
};

export default SwapInterface;

const TokenTag = ({ nft, size, isToggle, index }) => {
    const [isSelected, setIsSelected] = useState(false);
    const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom);
    const [, addItemToCart] = useAtom(addCartItemAtom);

    useEffect(() => {
        !isSelected ? removeItemFromCart(nft) : addItemToCart(nft);
    }, [isSelected]);

    useEffect(() => {
        setIsSelected(isToggle[index]);
    }, [isToggle]);

    return (
        <Tag
            _hover={{
                bg: 'extraLightGold',
            }}
            h="30px"
            minW={'120px'}
            transition=".2s all"
            cursor={'pointer'}
            bg={isSelected ? 'lightGold' : 'white'}
            border="1px"
            borderColor={'gold'}
            size={size}
            onClick={() => setIsSelected(!isSelected)}
        >
            <TagLabel transition="all 1s" m={isSelected ? '0' : '0 auto'} color={'black'}>
                {nft.name}
            </TagLabel>
            {isSelected && (
                <TagCloseButton
                    transition="all 1s"
                    opacity={isSelected ? 0.7 : 0}
                    height={isSelected ? '10px' : '0px'}
                    w={isSelected ? '10px' : '0px'}
                />
            )}
        </Tag>
    );
};

const Input = ({ isConnected }) => {
    return (
        <Card
            gridColumn={'span 1'}
            bg="white"
            borderRadius={'lg'}
            position="relative"
            border="1px"
            borderColor="border"
            shadow={'none'}
            w={'100%'}
            p={cardPadding.xl}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <MyNfts isConnected={isConnected} />
            <SelectedNfts isConnected={isConnected} />
            <FakeArrowButton />
        </Card>
    );
};

const Output = ({ isConnected }) => {
    return (
        <Card
            border="1px"
            borderColor="border"
            borderRadius={'lg'}
            bg="white"
            shadow={'none'}
            p={cardPadding.xl}
            sx={{ gridTemplateRows: 'repeat(1, 1fr)' }}
            gap={[3]}
        >
            <OutputOverview isConnected={isConnected} />
            <OutputDetails isConnected={isConnected} />
        </Card>
    );
};

const FakeArrowButton = () => {
    return (
        <Box
            h="45px"
            w="45px"
            overflow={'visible'}
            bottom={'-28px'}
            display={'flex'}
            justifySelf={'center'}
            alignSelf={'center'}
            alignItems={'center'}
            position={'absolute'}
            justifyContent={'center'}
            border={'4px'}
            borderColor={'bg'}
            p={0}
            m={0}
            borderRadius={200}
            bg="white"
            zIndex={2}
        >
            <Image width={'17px'} height={'17px'} src={Arrow} alt="arrow" />
        </Box>
    );
};

const OutputOverview = ({ isConnected }) => {
    const [weight] = useAtom(getTotalCartWeightAtom);
    const minted = weight * 100;
    return (
        <Card shadow="none" border="1px" borderColor="border">
            <CardBody>
                <HStack justifyContent="space-between">
                    <Box color={isConnected ? 'black' : 'secondaryText'}>You will receive</Box>
                    <HStack color={isConnected ? 'black' : 'secondaryText'}>
                        <Text>{minted.toString()}</Text>&nbsp;
                        <TokenSign />
                    </HStack>
                </HStack>
            </CardBody>
        </Card>
    );
};

const OutputDetails = ({ isConnected }) => {
    return (
        <Accordion allowToggle>
            <AccordionItem isDisabled={!isConnected} border="0">
                <AccordionButton
                    h="60px"
                    bg="bg"
                    border="1px"
                    borderColor="border"
                    borderStartEndRadius={'md'}
                    borderStartStartRadius={'md'}
                    disabled="flex"
                    justifyContent={'space-between'}
                >
                    <Text>Transaction Details</Text>
                    <AccordionIcon />
                </AccordionButton>
                <AccordionPanel
                    bg="bg"
                    borderEndEndRadius={'md'}
                    borderEndStartRadius={'md'}
                    border={'1px'}
                    borderColor={'border'}
                    borderTop={0}
                >
                    <TransactionDetailsTable />
                </AccordionPanel>
            </AccordionItem>
        </Accordion>
    );
};

const TransactionDetailsTable = () => {
    const [weight] = useAtom(getTotalCartWeightAtom);
    const [cart] = useAtom(getCartAtom);

    const minted = weight * 100;
    const fees = weight * 100 * 0.01;

    const Row = (props) => <Tr {...props}>{props.children}</Tr>;

    const Cell = ({ children, r }) => (
        <Td p={'5px'} sx={{ width: '50%' }} border={0} textAlign={r ? 'right' : 'left'}>
            <Box
                sx={{
                    display: 'flex',
                    width: '100%',
                    justifyContent: r ? 'flex-end' : 'flex-start',
                }}
            >
                {children}
            </Box>
        </Td>
    );

    return (
        <TableContainer color={'secondaryText'} fontSize={'14px'}>
            <Table>
                <Tbody>
                    <Row>
                        <Cell>Total number of NFTs selected</Cell>
                        <Cell r>{cart.length} NFTs</Cell>
                    </Row>
                    <Row>
                        <Cell>Total weight</Cell>
                        <Cell r>{weight} g</Cell>
                    </Row>
                    <Row>
                        <Cell>Swapped Amount</Cell>
                        <Cell r>
                            <HStack>
                                <Text>{minted} </Text>
                                <TokenSign />
                            </HStack>
                        </Cell>
                    </Row>
                    <Row>
                        <Cell>
                            <HStack>
                                <Text>Conversion fee (1%)</Text>
                                <Tooltip
                                    label="A conversion fee is deducted because the NFTs are running on the Origyn NFT standard. The Origyn NFT standard inherently protects creator royalties which are 1% for the GLD NFTs. Therefore, the fees are also present for the swapping of the GLDT."
                                    fontSize="sm"
                                >
                                    <InfoIcon />
                                </Tooltip>
                            </HStack>
                        </Cell>
                        <Cell r>
                            <HStack>
                                <Text>- {fees}</Text> <TokenSign />
                            </HStack>
                        </Cell>
                    </Row>
                    <Row>
                        <Cell>
                            <HStack>
                                <Text>Fee compensation</Text>
                                <Tooltip
                                    label="The conversion fee of the first 100 Million GLDT swapped are refunded."
                                    fontSize="sm"
                                >
                                    <InfoIcon />
                                </Tooltip>
                            </HStack>
                        </Cell>
                        <Cell r>
                            <HStack>
                                <Text>+ {fees} </Text>
                                <TokenSign />
                            </HStack>
                        </Cell>
                    </Row>
                    <Row>
                        <Cell></Cell>
                    </Row>
                    <Row borderTop="1px" borderColor="border" pt={2} mt={2} color="black">
                        <Cell>Total received</Cell>
                        <Cell r>
                            <HStack>
                                <Text>{minted}</Text>
                                <Tooltip>
                                    <TokenSign />
                                </Tooltip>
                            </HStack>
                        </Cell>
                    </Row>
                </Tbody>
            </Table>
        </TableContainer>
    );
};

const MyNfts = ({ isConnected }) => {
    const [connected, setConnected] = useState(isConnected);
    const [isLoading, setIsloading] = useState(false);
    useEffect(() => {
        setConnected(isConnected);
    }, [isConnected]);

    return (
        <Accordion allowToggle>
            <AccordionItem isDisabled={!connected || isLoading} border="0">
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
                    <Box>Select from my NFTs</Box>
                    {isLoading ? <Spinner size="sm" ml={'1em'} /> : <AccordionIcon />}
                </AccordionButton>
                {connected && <MyNftsPanel setIsloading={setIsloading} />}
            </AccordionItem>
        </Accordion>
    );
};

const MyNftsPanel = ({ setIsloading }) => {
    const actors = useAllCanisters();
    const { nftsByW, isLoading } = useNft(actors);
    const [isAllSelected, setIsAllSelected] = useState([false, false, false, false]);

    useEffect(() => {
        setIsloading(isLoading);
    }, [isLoading]);

    const selectSameWeight = (index) => {
        const nextState = [...isAllSelected];
        nextState[index] = true;
        setIsAllSelected(nextState);
    };

    const unSelectSameWeight = (index) => {
        const nextState = [...isAllSelected];
        nextState[index] = false;
        setIsAllSelected(nextState);
    };

    return (
        <AccordionPanel
            border={'1px'}
            borderColor={'border'}
            borderTop={0}
            p={{ base: '10px', md: '20px' }}
            gap="2"
            display="grid"
            gridTemplateColumns={'repeat(2,1fr)'}
            bg="bg"
            borderEndEndRadius={'md'}
            borderEndStartRadius={'md'}
        >
            {!isLoading &&
                nftsByW.map(
                    (weight, i) =>
                        weight.length > 0 && (
                            <Card key={i} shadow={'none'}>
                                <CardHeader
                                    display={'flex'}
                                    justifyContent={'space-between'}
                                    p={{ base: '10px', md: '20px' }}
                                    pb="0px"
                                    color={'secondaryText'}
                                >
                                    <Stack
                                        w={'100%'}
                                        justifyContent={{ base: 'flex-start', md: 'space-between' }}
                                        direction={{ base: 'column', md: 'row' }}
                                    >
                                        <Text>GLD NFTs {Object.keys(gldNftCanisters)[i]}</Text>
                                        <Button
                                            onClick={() =>
                                                isAllSelected[i]
                                                    ? unSelectSameWeight(i)
                                                    : selectSameWeight(i)
                                            }
                                            bg="transparent"
                                            border={'1px'}
                                            w="fit-content"
                                            borderColor={'black'}
                                            _hover={{ backgroundColor: 'bg' }}
                                            fontWeight={400}
                                            size={'xs'}
                                        >
                                            {isAllSelected[i] ? 'Unselect All' : 'Select All'}
                                        </Button>
                                    </Stack>
                                </CardHeader>
                                <CardBody p={{ base: '10px', md: '20px' }}>
                                    {isLoading ? (
                                        <SkeletonToken />
                                    ) : (
                                        <HStack w={'100%'} wrap="wrap">
                                            {weight.map((e, j) => {
                                                return (
                                                    <TokenTag
                                                        size="md"
                                                        nft={e}
                                                        key={j}
                                                        isToggle={isAllSelected}
                                                        index={i}
                                                    />
                                                );
                                            })}
                                        </HStack>
                                    )}
                                </CardBody>
                            </Card>
                        ),
                )}
        </AccordionPanel>
    );
};

const SelectedNfts = ({ isConnected }) => {
    const [cart] = useAtom(getCartAtom);
    const [weight] = useAtom(getTotalCartWeightAtom);
    return (
        <Card shadow="none" border="1px" borderColor="border">
            <HStack justifyContent={'space-between'}>
                <CardHeader color={isConnected ? 'black' : 'secondaryText'}>Selected</CardHeader>
                <CardBody textAlign="right" color={'secondaryText'}>
                    {cart.length} NFTs selected, {weight} g
                </CardBody>
            </HStack>
        </Card>
    );
};

const SwapButton = ({ isConnected }) => {
    const { isOpen, onOpen, onClose } = useDisclosure();
    const [cart] = useAtom(cartAtom);

    return (
        <>
            <Button
                isDisabled={isConnected && cart.length > 0 ? false : true}
                onClick={onOpen}
                color="white"
                bg="black"
                borderRadius={'500px'}
                h="50px"
                _hover={{
                    color: 'white',
                    bg: 'black',
                }}
            >
                {!isConnected && 'Connect your wallet to swap'}
                {isConnected && cart.length > 0 && 'Swap'}
                {isConnected && cart.length < 1 && 'Select NFTs to start swap'}
            </Button>
            <ConfirmationDialog isOpen={isOpen} onClose={onClose} onOpen={onOpen} />
        </>
    );
};

const ConfirmationDialog = ({ isOpen, onClose }) => {
    const actors = useAllCanisters();
    const [cart] = useAtom(cartAtom);
    const [weight] = useAtom(getTotalCartWeightAtom);
    const [res, setRes] = useState();
    const [loading, setLoading] = useState(false);
    const [, removeAllFromCart] = useAtom(removeAllItemsInCartAtom);

    useEffect(() => {
        removeAllFromCart();
    }, []);
    const handleBatchOffer = async () => {
        setLoading(true);
        const res = await sendBatchOffer(actors, cart);
        setRes(res);
        setLoading(false);
        removeAllFromCart();
    };

    return (
        <Modal isOpen={isOpen} onClose={onClose} size="3xl" isCentered>
            <ModalOverlay />
            <ModalContent gridColumn={'span 12'} borderRadius="2xl">
                <ModalHeader fontSize={'1.2em'} fontWeight={400}>
                    Transaction confirmation
                </ModalHeader>
                <ModalBody>
                    {!res && !loading && (
                        <Box>
                            <Box></Box>
                            <HStack w="100%" textAlign="center" justify="center">
                                <Text textAlign="center">
                                    Send {weight}g of GLD NFT to receive {weight * 100} GLDT
                                </Text>
                                <TokenSign />
                            </HStack>
                        </Box>
                    )}
                    {loading && !res && (
                        <HStack justify={'center'} w="100%">
                            <Spinner size="md" /> <Text> awaiting response...</Text>
                        </HStack>
                    )}
                    {res && <BatchOfferResponse res={res} />}
                </ModalBody>
                <ModalFooter>
                    {!res && !loading && (
                        <Button
                            w="100%"
                            color="white"
                            bg="black"
                            borderRadius={'500px'}
                            h="50px"
                            _hover={{
                                color: 'white',
                                bg: 'black',
                            }}
                            _disabled={!res && !loading ? false : true}
                            onClick={(e) => handleBatchOffer()}
                        >
                            Confirm transaction
                        </Button>
                    )}
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

const BatchOfferResponse = ({ res, loading }) => {
    return !loading ? (
        <>
            {res?.map((el) => {
                return el?.map((e, i) => (
                    <Box pb={'20px'} key={i}>
                        {e.ok && (
                            <HStack>
                                <CheckCircleIcon color="green.300" />
                                <Text>{e.ok?.token_id}</Text>
                            </HStack>
                        )}
                        {e.err && (
                            <HStack>
                                <WarningIcon color="red.300" />
                                <Text>{e.err?.text}</Text>
                            </HStack>
                        )}
                    </Box>
                ));
            })}
            <Text>
                Swap request successfully sent. You can follow the progress on your&nbsp;
                <Link
                    href={'/my-account'}
                    style={{
                        textDecoration: 'underline',
                    }}
                >
                    Account Page
                </Link>
            </Text>
        </>
    ) : (
        <HStack justify={'center'}>
            <Spinner size="md" />
            <Text>Sending batch offer...</Text>
        </HStack>
    );
};

const SkeletonToken = () => {
    const count = 5;
    return (
        <HStack w={'100%'} wrap="wrap">
            {Array.from({ length: count }).map((e, i) => (
                <Skeleton height="25px" minW={'100px'} maxW={'100%'} key={i} />
            ))}
        </HStack>
    );
};
