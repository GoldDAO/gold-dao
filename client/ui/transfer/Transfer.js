import React, { useEffect, useState } from 'react';
import {
    Accordion,
    AccordionButton,
    AccordionItem,
    AccordionPanel,
    Box,
    Button,
    FormLabel,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    VStack,
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
    NumberInput,
    NumberInputField,
} from '@chakra-ui/react';
import { useConnect } from '@connect2ic/react';
import { cardPadding } from '@/theme/theme';
import { Input as TextInput } from '@chakra-ui/react';
import TokenSign from '../gldt/TokenSign';
const Transfer = () => {
    const { isConnected } = useConnect();
    return (
        <Card
            gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
            position={'relative'}
            shadow={['none']}
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
            <TransferButton />
        </Card>
    );
};

export default Transfer;

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
            <VStack alignSelf={'flex-start'} w={'100%'} justifyContent={'flex-start'}>
                <FormLabel color={'secondaryText'} alignSelf={'flex-start'} fontWeight={400}>
                    Principal ID or Account ID
                </FormLabel>
                <TextInput
                    w={'100%'}
                    size={'lg'}
                    isDisabled={!isConnected}
                    placeholder="0x000-000-000-000"
                />
            </VStack>
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
            <VStack alignSelf={'flex-start'} w={'100%'} justifyContent={'flex-start'}>
                <FormLabel color={'secondaryText'} alignSelf={'flex-start'} fontWeight={400}>
                    Amount
                </FormLabel>
                <NumberInput allowMouseWheel w={'100%'} isDisabled={!isConnected}>
                    <NumberInputField size={'lg'} placeholder="00"></NumberInputField>
                </NumberInput>
            </VStack>
        </Card>
    );
};

const TransferButton = ({ isConnected }) => {
    const { isOpen, onOpen, onClose } = useDisclosure();
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
                {!isConnected && 'Connect your wallet to transfer'}
                {isConnected && cart.length > 0 && 'Transfer'}
                {isConnected && cart.length < 1 && 'Select amount to transfer'}
            </Button>
        </>
    );
};
