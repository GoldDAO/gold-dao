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
    Tag,
    TagLabel,
    TagRightIcon,
    CircularProgress,
    Skeleton,
    HStack,
    Text,
} from '@chakra-ui/react';
import {
    addCartItemAtom,
    cartAtom,
    getCartAtom,
    getTotalCartWeightAtom,
    removeCartItemByIdAtom,
} from '@/atoms/cart';
import { useAllCanisters } from '@/query/hooks/useAllCanisters';
import { useNft } from '@/query/hooks/useNFTs';
import { useAtom } from 'jotai';
import { useConnect } from '@connect2ic/react';
import { sendBatchOffer } from '@/query/sendBatchOffer';
import Link from 'next/link';

const SwapInterface = () => {
    const { isConnected } = useConnect();
    return (
        <Card>
            <Input isConnected={isConnected} />
            <Output isConnected={isConnected} />
            <SwapButton isConnected={isConnected} />
        </Card>
    );
};

export default SwapInterface;

const TokenTag = ({ nft, size }) => {
    const [isSelected, setIsSelected] = useState(false);
    const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom);
    const [, addItemToCart] = useAtom(addCartItemAtom);

    useEffect(() => {
        !isSelected ? removeItemFromCart(nft) : addItemToCart(nft);
    }, [isSelected]);

    return (
        <Tag size={size} onClick={() => setIsSelected(!isSelected)}>
            <TagLabel>{nft.name}</TagLabel>
            {isSelected && <TagCloseButton />}
        </Tag>
    );
};

const Input = ({ isConnected }) => {
    return (
        <Card>
            <MyNfts isConnected={isConnected} />
            <SelectedNfts isConnected={isConnected} />
        </Card>
    );
};

const Output = ({ isConnected }) => {
    return (
        <Card>
            <OutputOverview isConnected={isConnected} />
            <OutputDetails isConnected={isConnected} />
        </Card>
    );
};

const OutputOverview = ({ isConnected }) => {
    const [weight] = useAtom(getTotalCartWeightAtom);
    const minted = weight * 100;
    return (
        <Card>
            <CardBody>
                <HStack justifyContent="space-between">
                    <Box>You will receive</Box>
                    <Box>{minted}&nbsp;GLDT</Box>
                </HStack>
            </CardBody>
        </Card>
    );
};

const OutputDetails = ({ isConnected }) => {
    return (
        <Accordion allowToggle>
            <AccordionItem isDisabled={!isConnected}>
                <AccordionButton>
                    <Box>Transaction Details</Box>
                </AccordionButton>
                <AccordionPanel>
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

    return (
        <TableContainer>
            <Table>
                <Tbody>
                    <Tr>
                        <Td>Total number of NFTs selected</Td>
                        <Td>{cart.length} NFTs</Td>
                    </Tr>
                    <Tr>
                        <Td>Total weight</Td>
                        <Td>{weight} g</Td>
                    </Tr>
                    <Tr>
                        <Td>Swapped Amount</Td>
                        <Td>{minted} GLDT</Td>
                    </Tr>
                    <Tr>
                        <Td>Conversion fee (1%)</Td>
                        <Td>{fees} GLDT</Td>
                    </Tr>
                    <Tr>
                        <Td>Fee compensation</Td>
                        <Td>{fees} GLDT</Td> const [weight] = useAtom(getTotalCartWeightAtom);
                    </Tr>
                    <Tr>
                        <Td>Total received</Td>
                        <Td>{minted}</Td>
                    </Tr>
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
            <AccordionItem isDisabled={!connected}>
                <AccordionButton>
                    <Box>Select from my NFTs</Box>
                    {isLoading && <Spinner size="sm" ml={'1em'} />}
                </AccordionButton>
                {connected && <MyNftsPanel setIsloading={setIsloading} isLoading={isLoading} />}
            </AccordionItem>
        </Accordion>
    );
};

const MyNftsPanel = ({ setIsloading, isLoading }) => {
    const actors = useAllCanisters();
    const nfts = useNft(actors);
    const weights = [1, 10, 100, 1000];

    useEffect(() => {
        setIsloading(nfts.isLoading);
    }, [nfts.isLoading]);

    return (
        <AccordionPanel>
            {weights.map((weight, i) => (
                <Card key={i}>
                    <CardHeader>GLDNFT {weight}g</CardHeader>
                    <CardBody>
                        {isLoading ? (
                            <SkeletonToken />
                        ) : (
                            nfts.nfts.map(
                                (e, i) =>
                                    e.weight === weight && <TokenTag size="sm" nft={e} key={i} />,
                            )
                        )}
                    </CardBody>
                </Card>
            ))}
        </AccordionPanel>
    );
};

const SelectedNfts = () => {
    const [cart] = useAtom(getCartAtom);
    const [weight] = useAtom(getTotalCartWeightAtom);
    return (
        <Card>
            <HStack justifyContent={'space-between'}>
                <CardHeader>Selected</CardHeader>
                <CardBody textAlign="right">
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
            <Button isDisabled={isConnected && cart.length > 0 ? false : true} onClick={onOpen}>
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

    const handleBatchOffer = async () => {
        setLoading(true);
        const res = await sendBatchOffer(actors, cart);
        setRes(res);
        setLoading(false);
    };

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>Transaction confirmation</ModalHeader>
                {!res && !loading && (
                        <>
                            <Box>
                                Send {cart.length} NFTs ,({weight} g)
                            </Box>
                            <Button onClick={(e) => handleBatchOffer()}>Confirm transaction</Button>
                        </>,
                    )}
                {loading && <Spinner size="sm" />}
                {res && <BatchOfferResponse />}
            </ModalContent>
        </Modal>
    );
};

const BatchOfferResponse = ({ res, loading }) => {
    return (
        <>
            {!loading ? (
                <>
                    {res?.map((e) => {
                        return e?.map((e, i) => (
                            <Box>
                                <Box>{e.ok.token_id}</Box>
                            </Box>
                        ));
                    })}
                    <Text>
                        Batch offer successfully sent, you can follow the progress on your
                        <Link href={'/my-account'}> Account Page</Link>
                    </Text>
                </>
            ) : (
                <Box>
                    <Spinner />
                    Sending batch offer...
                </Box>
            )}
        </>
    );
};

const SkeletonToken = () => {
    const count = 5;
    return (
        <HStack w={'100%'} wrap="wrap">
            {Array.from({ length: count }).map(() => (
                <Skeleton height="25px" minW={'100px'} maxW={'100%'} />
            ))}
        </HStack>
    );
};
