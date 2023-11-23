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
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { useNft } from '@utils/hooks/gldnfts/useNFTs';
import { useAtom } from 'jotai';
import { useConnect } from '@connect2ic/react';
import { sendBatchOffer } from '@utils/queries/sendBatchOffer';
import Link from 'next/link';
import TokenSign from '@ui/gldt/TokenSign';
import { CheckCircleIcon, ChevronDownIcon, InfoIcon, WarningIcon } from '@chakra-ui/icons';
import Image from 'next/image';
import Arrow from '/public/images/arrow.svg';
import { gldNftCanisters } from '@utils/agents';
import { cardPadding } from '@ui/theme';
import { useDialog } from '@connect2ic/react';

const SwapInterface = ({ setIsConnected }) => {
    const { isConnected } = useConnect();
    const { open, isOpen } = useDialog();

    useEffect(() => {
        setIsConnected(isConnected);
    }, [isConnected]);

    const Overlay = () => {
        return (
            <Box
                cursor={'pointer'}
                onClick={open}
                sx={{
                    width: '100vw',
                    height: '100vh',
                    position: 'absolute',
                    top: 0,
                    color: 'transparent',
                    left: 0,
                    display: isConnected ? 'none' : 'block',
                    zIndex: isOpen ? -1 : 100,
                }}
            ></Box>
        );
    };

    return (
        <>
            <Overlay />
            <Card
                gridColumn={['1/13', '1/13', '2/12', '3/11', '4/10']}
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
                borderRadius={['lg', 'lg', 'lg', 'xl']}
            >
                <Input isConnected={isConnected} />
                <Output isConnected={isConnected} />
                <SwapButton isConnected={isConnected} />
            </Card>
        </>
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
            opacity={isSelected ? '1' : '.6'}
            bg={isSelected ? 'lightGold' : 'lightGold'}
            border={isSelected ? '1px' : '0px'}
            borderColor={'darkGold'}
            justifyContent={'center'}
            size={size}
            onClick={() => setIsSelected(!isSelected)}
        >
            <HStack justify={'space-between'}>
                <TagLabel
                    transition="all 1s"
                    m={isSelected ? '0' : '0 auto'}
                    color={'veryDarkGold'}
                >
                    {nft.name}
                </TagLabel>
                {isSelected && (
                    <TagCloseButton
                        color={'veryDarkGold'}
                        transition="all 1s"
                        opacity={isSelected ? 0.7 : 0}
                        height={isSelected ? '10px' : '0px'}
                        w={isSelected ? '10px' : '0px'}
                    />
                )}
            </HStack>
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
            gap={[2]}
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
            gap={[2]}
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
        <Card
            shadow="none"
            border="1px"
            borderColor="border"
            borderEndStartRadius={0}
            borderEndEndRadius={0}
            h="60px"
        >
            <CardBody py="0">
                <HStack justifyContent="space-between" alignItems={'center'} height={'100%'}>
                    <Box color={'secondaryText'}>You will receive</Box>
                    <HStack color={'secondaryText'}>
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
                    borderRadius={'md'}
                    borderStartStartRadius={0}
                    borderStartEndRadius={0}
                    disabled="flex"
                    justifyContent={'space-between'}
                >
                    <Text fontSize={'16px'}>Transaction Details</Text>
                    <AccordionIcon />
                </AccordionButton>
                <AccordionPanel bg="bg" border={'1px'} borderColor={'border'} borderTop={0}>
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
                                <Text fontSize={'14px'}>{minted} </Text>
                                <TokenSign />
                            </HStack>
                        </Cell>
                    </Row>
                    <Row>
                        <Cell>
                            <HStack>
                                <Text fontSize={'14px'}>Conversion fee (1%)</Text>
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
                                <Text fontSize={'14px'}>- {fees}</Text> <TokenSign />
                            </HStack>
                        </Cell>
                    </Row>
                    <Row>
                        <Cell>
                            <HStack>
                                <Text fontSize={'14px'}>Fee compensation</Text>
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
                                <Text fontSize={'14px'}>+ {fees} </Text>
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
                                <Text fontSize={'16px'}>{minted}</Text>
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
                    <Text fontSize={'16px'}>Select from my NFTs</Text>
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
            p={'3px'}
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
                            <Card key={i} shadow={'none'} borderRadius={0}>
                                <CardHeader
                                    display={'flex'}
                                    justifyContent={'space-between'}
                                    p={'10px'}
                                    color={'secondaryText'}
                                    borderBottom={'1px'}
                                    borderBottomColor="bg"
                                >
                                    <Stack
                                        w={'100%'}
                                        direction="column"
                                        justifyContent={{ base: 'flex-start', md: 'space-between' }}
                                    >
                                        <Text fontSize={'14px'} color="black">
                                            GLD NFTs {Object.keys(gldNftCanisters)[i]}
                                        </Text>
                                    </Stack>
                                </CardHeader>
                                <CardBody p={'10px'} pt="10px">
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
                                <CardFooter px="10px" pt="0" pb="10px">
                                    <Button
                                        onClick={() =>
                                            isAllSelected[i]
                                                ? unSelectSameWeight(i)
                                                : selectSameWeight(i)
                                        }
                                        bg="transparent"
                                        border={'1px'}
                                        w="fit-content"
                                        borderColor={'secondaryText'}
                                        _hover={{ backgroundColor: 'bg' }}
                                        fontWeight={400}
                                        size={'xs'}
                                        outline={'none'}
                                    >
                                        {isAllSelected[i] ? 'Unselect All' : 'Select All'}
                                    </Button>
                                </CardFooter>
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
        <Card
            shadow="none"
            h="60px"
            border="1px"
            borderColor="border"
            borderTopStartRadius={0}
            borderStartEndRadius={0}
        >
            <HStack justifyContent={'space-between'}>
                <CardHeader color={'secondaryText'}>Selected</CardHeader>
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
    const [err, setErr] = useState(0);
    const [succ, setSucc] = useState(0);
    const [kycWarn, setKycWarn] = useState(false);

    useEffect(() => {
        setSucc(0);
        setErr(0);
        setKycWarn(false);
        res?.map((el) => {
            return el?.map((e, i) => {
                e.err?.text && e.err?.text === 'kyc fail' && setKycWarn(true);
                e.err && setErr((prev) => prev + 1);
                e.ok && setSucc((prev) => prev + 1);
            });
        });
    }, [res]);

    const errorTexts = {
        'token is non-transferable': 'Token is non-transferable',
        'kyc fail': `Your account hasn't been KYC'd`,
    };

    const KYCWarnText = `Your account hasn't been KYC'd and is not eligible to swap GLD NFT for GLDT. Please go to yumi.io to get verified.`;

    return !loading ? (
        <>
            {res?.map((el) => {
                return el?.map((e, i) => (
                    <Box key={i}>
                        {e.ok && (
                            <HStack>
                                <CheckCircleIcon color="green.400" />
                                <Text>{e.ok?.token_id}</Text>
                            </HStack>
                        )}
                        {e.err && (
                            <HStack>
                                <WarningIcon color="red.400" />
                                <Text>{errorTexts[e.err?.text] || e.err?.text}</Text>
                            </HStack>
                        )}
                    </Box>
                ));
            })}
            <Text pt="20px">
                {err > 0 && (
                    <HStack>
                        <Box
                            height={'20px'}
                            width={'20px'}
                            bg="red.400"
                            borderRadius={'20px'}
                            display={'flex'}
                            alignItems={'center'}
                            justifyContent={'center'}
                            color="white"
                            fontSize={'12px'}
                        >
                            {err}
                        </Box>
                        <Text>Errors</Text>
                    </HStack>
                )}
                {succ > 0 && (
                    <HStack>
                        <Box
                            height={'20px'}
                            width={'20px'}
                            bg="green.400"
                            borderRadius={'20px'}
                            display={'flex'}
                            alignItems={'center'}
                            justifyContent={'center'}
                            color="white"
                            fontSize={'12px'}
                        >
                            {succ}
                        </Box>
                        <Text>NFTs successfully swap</Text>
                    </HStack>
                )}
                {kycWarn && <Box pt="20px">{KYCWarnText}</Box>}
                <Box pt="20px">
                    <Link
                        href={'/my-account'}
                        style={{
                            textDecoration: 'underline',
                            marginTop: '20px',
                        }}
                    >
                        Account Page
                    </Link>
                </Box>
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
