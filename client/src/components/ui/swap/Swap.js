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
    return (
        <Card>
            <CardBody>
                <Box>You will receive</Box>
                <Box>GLDT</Box>
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
                        <Td>{fees} GLDT</Td>
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
    const actors = useAllCanisters();
    const nfts = useNft(actors);
    return (
        <Accordion allowToggle>
            <AccordionItem isDisabled={!isConnected || nfts.isLoading ? true : false}>
                <AccordionButton>
                    <Box>Select from my NFTs</Box>
                    {nfts.isLoading && isConnected ? (
                        <Box>
                            <Spinner />
                        </Box>
                    ) : null}
                </AccordionButton>
                {isConnected && <MyNftsPanel nfts={nfts} />}
            </AccordionItem>
        </Accordion>
    );
};

const MyNftsPanel = ({ nfts }) => {
    return (
        <AccordionPanel>
            {nfts.nfts.map((e, i) => (
                <TokenTag size="sm" nft={e} key={i} />
            ))}
        </AccordionPanel>
    );
};

const SelectedNfts = () => {
    const [cart] = useAtom(getCartAtom);
    const [weight] = useAtom(getTotalCartWeightAtom);
    return (
        <Card>
            <CardHeader>Selected</CardHeader>
            <CardBody>
                {cart.length} NFTs selected, {weight} g
            </CardBody>
        </Card>
    );
};

const SwapButton = ({ isConnected }) => {
    const { isOpen, onOpen, onClose } = useDisclosure();

    return (
        <>
            <Button isDisabled={!isConnected} onClick={onOpen}>
                {isConnected ? 'Swap' : 'Connect You wallet to swap'}{' '}
            </Button>
            <ConfirmationDialog isOpen={isOpen} onClose={onClose} onOpen={onOpen} />
        </>
    );
};

const ConfirmationDialog = ({ isOpen, onClose }) => {
    const actors = useAllCanisters();
    const [cart] = useAtom(cartAtom);

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>Transaction confirmation</ModalHeader>
                <TransactionDetailsTable />
                <Button onClick={(e) => sendBatchOffer(actors, cart)}>Confirm transaction</Button>
            </ModalContent>
        </Modal>
    );
};
