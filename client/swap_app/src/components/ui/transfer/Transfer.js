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
    useToast,
} from '@chakra-ui/react';
import { useCanister, useConnect } from '@connect2ic/react';
import { cardPadding } from '@ui/theme';
import { Input as TextInput } from '@chakra-ui/react';
import { transfer } from '@utils/queries/transfer';

const Transfer = ({ setIsConnected }) => {
    const { isConnected } = useConnect();
    useEffect(() => {
        setIsConnected(isConnected);
    }, [isConnected]);

    const [amount, setAmount] = useState();
    const [to, setTo] = useState();

    return (
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
            borderRadius={'2xl'}
        >
            <Output isConnected={isConnected} setAmount={setAmount} />
            <Input isConnected={isConnected} setTo={setTo} />
            <TransferButton isConnected={isConnected} to={to} amount={amount} />
        </Card>
    );
};

export default Transfer;

const Input = ({ isConnected, setTo }) => {
    const handleChange = (e) => {
        setTo(e.target.value);
    };
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
                    onChange={handleChange}
                />
            </VStack>
        </Card>
    );
};

const Output = ({ isConnected, setAmount }) => {
    const handleChange = (e) => {
        setAmount(e.target.value);
    };
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
                    <NumberInputField
                        size={'lg'}
                        onChange={handleChange}
                        placeholder="00"
                    ></NumberInputField>
                </NumberInput>
            </VStack>
        </Card>
    );
};

const TransferButton = ({ isConnected, amount, to }) => {
    const gldtLedgerActor = useCanister('gldtLedgerCanister')[0];
    const [isLoading, setIsLoading] = useState(false);
    const toast = useToast({
        position: 'bottom',
    });

    const handleTransfer = async () => {
        setIsLoading(true);
        const res = await transfer(amount, to, gldtLedgerActor);
        console.log('res', res);
        if (res?.Ok) {
            toast({
                title: 'Success',
                description: 'Transaction Sent',
            });
        } else {
            toast({
                title: 'Failed',
                description: 'Something went wrong',
            });
        }
        setIsLoading(false);
    };

    return (
        <>
            <Button
                isDisabled={isLoading ? true : isConnected ? false : true}
                onClick={handleTransfer}
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
                {isConnected && !isLoading && 'Transfer'}
                {isLoading && 'Sending transaction...'}
            </Button>
        </>
    );
};
