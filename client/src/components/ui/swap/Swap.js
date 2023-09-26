import React, { useEffect } from 'react';
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
    TagCloseButton,
} from '@chakra-ui/react';
import { Tag, TagLabel, TagRightIcon } from '@chakra-ui/react';
import {
    addAllItemsAtom,
    getCartAtom,
    removeAllItemsInCartAtom,
    removeCartItemByIdAtom,
} from '@/atoms/cart';
import { useAllCanisters } from '@/hooks/useAllCanisters';
import { useNft } from '@/hooks/useNFTs';
import { useAtom } from 'jotai';
import { useConnect } from '@connect2ic/react';

const SwapInterface = () => {
    return (
        <Card>
            <Input />
            <Output />
            <SwapButton />
        </Card>
    );
};

export default SwapInterface;

const TokenTag = ({ label, size }) => {
    const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom);
    return (
        <Tag size={size}>
            <TagLabel>{label}</TagLabel>
            <TagCloseButton onClick={() => removeItemFromCart(label)} />
        </Tag>
    );
};

const Input = () => {
    return (
        <Card>
            <MyNfts />
            <SelectedNfts />
        </Card>
    );
};

const Output = () => {
    return (
        <Card>
            <OutputOverview />
            <OutputDetails />
        </Card>
    );
};

const OutputDetails = () => {
    return <></>;
};

const OutputOverview = () => {
    return <></>;
};

const MyNfts = () => {
    const nfts = useNft();

    useEffect(() => {
        console.log('nfts', nfts);
    }, [nfts]);

    const { isConnected } = useConnect();

    return (
        <Accordion allowToggle>
            <AccordionItem>
                <AccordionButton>
                    <Box>Select from my NFTs</Box>
                </AccordionButton>
                {isConnected && <AccordionPanel></AccordionPanel>}
            </AccordionItem>
        </Accordion>
    );
};

const SelectedNfts = () => {
    const [cart] = useAtom(getCartAtom);
    return (
        <Card>
            <CardHeader>Selected</CardHeader>
            <CardBody>
                {cart.map((token, index) => {
                    <TokenTag label={token.token_id} key={index} />;
                })}
            </CardBody>
            <CardFooter>{cart.length} NFTs selected</CardFooter>
        </Card>
    );
};

const SwapButton = () => {
    return <Button></Button>;
};
