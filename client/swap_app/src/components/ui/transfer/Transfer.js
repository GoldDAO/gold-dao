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
    InputGroup,
    InputRightAddon,
} from '@chakra-ui/react';
import { useCanister, useConnect } from '@connect2ic/react';
import { cardPadding } from '@ui/theme';
import { Input as TextInput } from '@chakra-ui/react';
import { transfer } from '@utils/queries/transfer';
import TokenSign from '@ui/gldt/TokenSign';
import Link from 'next/link';

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
            borderRadius={['lg', 'lg', 'lg', 'xl']}
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
                <FormLabel alignSelf={'flex-start'} fontWeight={400} pl="5px" mb="0">
                    To
                </FormLabel>
                <TextInput
                    w={'100%'}
                    size={'lg'}
                    isDisabled={!isConnected}
                    height={'50px'}
                    maxH={'65px'}
                    placeholder="Principal ID"
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
                <FormLabel alignSelf={'flex-start'} fontWeight={400} pl="5px" mb="0">
                    Input Amount
                </FormLabel>
                <InputGroup>
                    <NumberInput allowMouseWheel w={'100%'} isDisabled={!isConnected}>
                        <NumberInputField
                            size={'lg'}
                            onChange={handleChange}
                            placeholder="100"
                            height={'50px'}
                            borderTopRightRadius={0}
                            borderBottomRightRadius={0}
                            defaultValue={100}
                        />
                    </NumberInput>
                    <InputRightAddon bg="white" height={'50px'}>
                        <TokenSign />
                    </InputRightAddon>
                </InputGroup>
            </VStack>
        </Card>
    );
};
const TransferButton = ({ isConnected, amount, to }) => {
    const gldtLedgerActor = useCanister('gldtLedgerCanister')[0];
    const [isLoading, setIsLoading] = useState(false);
    const [isEnable, setIsEnable] = useState(true);

    const isPrincipal = (str) => {
        const regex = /^([a-zA-Z0-9]{5}-){10}[a-zA-Z0-9]{3}$/;
        return regex.test(str);
    };

    useEffect(() => {
        if (amount > 0 && isPrincipal(to) && isConnected) {
            setIsEnable(true);
        } else {
            setIsEnable(false);
        }
    }, [amount, to, isConnected]);

    const toast = useToast({
        position: 'bottom',
    });

    const handleTransfer = async () => {
        setIsLoading(true);
        const res = await transfer(amount, to, gldtLedgerActor);
        const env = process.env.DFX_NETWORK;
        const prefix = env === 'ic' ? '' : 'staging';
        if (res?.Ok) {
            toast({
                title: 'Success',
                description: (
                    <Link
                        href={`https://${prefix}.explorer.gldt.org/transaction/${res.Ok}`}
                        target="_blank"
                        style={{
                            textDecoration: 'underline',
                        }}
                    >
                        Transaction {parseInt(res.Ok)} Sent
                    </Link>
                ),
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
                isDisabled={!isEnable}
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
