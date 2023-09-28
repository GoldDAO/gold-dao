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
    Th,
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
import TokenSign from '../gldt/TokenSign';

const SwapInterface = () => {
    const { isConnected } = useConnect();
    return (
        <Card 
            p={[2, 2, 2 , 4]} 
            shadow={['md','lg']} 
            sx={{gridColumn: 'span 12'}} 
            bg='bg' 
            display='grid'
            gridTemplateRows={'repeat(1, 1fr)'}
            gap='3'
            borderRadius={'2xl'}
        >
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
        <Tag bg={isSelected ? 'lightGold' : 'white'} border='1px' borderColor={'gold'} size={size} onClick={() => setIsSelected(!isSelected)}> 
            <TagLabel color={'black'}>{nft.name}</TagLabel>
            {isSelected && <TagCloseButton />}
        </Tag>
    );
};

const Input = ({ isConnected }) => {
    return (
        <Card 
        bg='white'  
        borderRadius={'lg'}
        border="1px"
        borderColor="border"
        shadow={'none'}
        p={[2,2,3,4,6]} 
        sx={{gridTemplateRows: 'repeat(1, 1fr)'}} 
        gap={[3]}>
            <MyNfts isConnected={isConnected} />
            <SelectedNfts isConnected={isConnected} />
        </Card>
    );
};

const Output = ({ isConnected }) => {
    return (
        <Card
        border="1px"
        borderColor="border"
        borderRadius={'lg'}
        bg='white'  
        shadow={'none'}
        p={[2,2,3,4,6]} 
        sx={{gridTemplateRows: 'repeat(1, 1fr)'}} 
        gap={[3]}>
            <OutputOverview isConnected={isConnected} />
            <OutputDetails isConnected={isConnected} />
        </Card>
    );
};

const OutputOverview = ({ isConnected }) => {
    const [weight] = useAtom(getTotalCartWeightAtom);
    const minted = weight * 100;
    return (
        <Card
            shadow='none'
            border="1px"
            borderColor="border">
            <CardBody>
                <HStack justifyContent="space-between">
                    <Box color={isConnected ? 'black' : 'secondaryText'}>You will receive</Box>
                    <HStack color={isConnected ? 'black' : 'secondaryText'}><Text>{minted.toString()}</Text>&nbsp;<TokenSign /></HStack>
                </HStack>
            </CardBody>
        </Card>
    );
};

const OutputDetails = ({ isConnected }) => {
    return (
        <Accordion allowToggle>
            <AccordionItem isDisabled={!isConnected}  border='0' >
                <AccordionButton                
                h='60px'
                bg='bg' 
                border='1px' 
                borderColor='border' 
                borderStartEndRadius={'md'} 
                borderStartStartRadius ={'md'}>
                    <Box>Transaction Details</Box>
                </AccordionButton>
                <AccordionPanel bg='bg'borderEndEndRadius={'md'} borderEndStartRadius={'md'} border={'1px'} borderColor={'border'} borderTop={0}>
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
    const fees = weight * 100 * 0.01

    const Row = (props) => <Tr {...props }>{props.children}</Tr>

    const Cell = ({children, r }) => <Td p={'5px'} sx={{ width: '50%'}} border={0} textAlign={r ? 'right' : 'left'} >
        <Box sx={{display: 'flex', width: '100%', justifyContent: r ? 'flex-end' : 'flex-start'}}>{children}</Box></Td>

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
                        <Cell r><HStack><Text>{minted} </Text><TokenSign /></HStack></Cell>
                    </Row>
                    <Row>
                        <Cell>Conversion fee (1%)</Cell>
                        <Cell r><HStack><Text>{fees}</Text> <TokenSign /></HStack></Cell>
                    </Row>
                    <Row>
                        <Cell>Fee compensation</Cell>
                        <Cell r><HStack><Text>{fees} </Text><TokenSign /></HStack></Cell> 
                    </Row>
                    <Row><Cell></Cell></Row>
                    <Row borderTop='1px' borderColor='border' pt={2} mt={2} color='black'>
                        <Cell>Total received</Cell>
                        <Cell r>{minted}</Cell>
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
            <AccordionItem isDisabled={!connected} border='0' >
                <AccordionButton 
                h='60px'
                bg='bg' 
                border='1px' 
                borderColor='border' 
                borderStartEndRadius={'md'} 
                borderStartStartRadius ={'md'}>
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
        <AccordionPanel 
            border={'1px'} 
            borderColor={'border'} 
            borderTop={0}
            gap='2'
            display='grid'
            gridTemplateColumns={'repeat(2,1fr)'}
            bg='bg' 
            borderEndEndRadius={'md'} 
            borderEndStartRadius={'md'}
        >
            {weights.map((weight, i) => (
                <Card key={i} shadow={'none'}>
                    <CardHeader color={'secondaryText'}>GLDNFT {weight}g</CardHeader>
                    <CardBody>
                        {isLoading ? (
                            <SkeletonToken />
                        ) : (
                            <HStack w={'100%'} wrap="wrap">
                            {nfts.nfts.map(
                                (e, i) =>
                                    e.weight === weight && <TokenTag size="lg" nft={e} key={i} />,
                            )}
                            </HStack>
                        )}
                    </CardBody>
                </Card>
            ))}
        </AccordionPanel>
    );
};

const SelectedNfts = ({isConnected}) => {
    const [cart] = useAtom(getCartAtom);
    const [weight] = useAtom(getTotalCartWeightAtom);
    return (
        <Card
            shadow='none'
            border="1px"
            borderColor="border"
        >
            <HStack justifyContent={'space-between'}>
                <CardHeader color={isConnected ? 'black' : 'secondaryText'} >Selected</CardHeader>
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
                h='50px' 
                _hover={{
                color: 'white',
                bg: 'black'
            }}>
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
