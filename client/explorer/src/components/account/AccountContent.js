import {
    Box,
    Button,
    Card,
    Flex,
    HStack,
    Heading,
    Select,
    Table,
    TableContainer,
    Tbody,
    Td,
    Text,
    Thead,
    Tr,
    VStack,
} from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { useSubaccounts } from '@utils/hooks/ledgerIndexer/useSubaccount';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import { useBalance } from '@utils/hooks/ledgerIndexer/useBalance';
import { useHistory } from '@utils/hooks/ledgerIndexer/useHistory';
import { formatAmount } from '@utils/misc/format';
import useGLDTbalance from '@utils/hooks/useGLDTbalance';
import PrincipalFormat from '../Principal';
import Timestamp from '@ui/tooltip/timeStamp';
import TokenSign from '@ui/gldt/TokenSign';
import { ArrowBackIcon, ArrowForwardIcon } from '@chakra-ui/icons';
import AccountTitle from './AccountTitle';
import { buf2hex } from '@/utils/buf2hex';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { Principal } from '@dfinity/principal';

const AccountContent = ({ id, subAccount }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [currentSub, setCurrentSub] = useState();
    const [index, setIndex] = useState({
        last: null,
        first: null,
    });
    const [last, setlast] = useState([]);
    const [action, setAction] = useState();
    const { history } = useHistory(id, currentPage, action, index, last, currentSub);
    const { subaccounts } = useSubaccounts(id);
    const { balance } = useBalance(id, currentSub ? currentSub : subAccount);
    const router = useRouter();

    useEffect(() => {
        console.log('history', history);
        if (history?.Ok?.transactions.length > 0) {
            setIndex({
                last: history?.Ok?.transactions[history.Ok.transactions.length - 1].id,
                first: history?.Ok?.transactions[0].id,
            });
        }
    }, [history]);

    // useEffect(() => {
    //     last.push(history?.Ok?.transactions[history.Ok.transactions.length - 1].id);
    // }, [history]);

    const toggleChange = (e) => {
        setCurrentSub(e.target.value);
    };

    useEffect(() => {
        if (currentSub) {
            router.push(`/account/${id}?subaccount=${currentSub}`);
        }
    }, [currentSub, id, router]);

    return (
        <VStack
            alignItems={'flex-start'}
            gridColumn={['1/13', '1/13', '3/11', '3/11']}
            spacing="100px"
            my="100px"
        >
            <AccountTitle
                data={{
                    label: 'AccountID',
                    id: '20e43f0bd4f09346ed0bfd7006ed3a0df564c1a1e6eb483f8315d592f872e98f',
                }}
            />
            <Select size="md" onChange={toggleChange} placeholder={subAccount} value={currentSub}>
                {subaccounts.map((e, i) => (
                    <option key={i} value={buf2hex(e)}>
                        {buf2hex(e)}
                    </option>
                ))}
            </Select>
            <Heading fontWeight={{}} as="h1">
                <PrincipalFormat principal={id} />
            </Heading>
            <Card shadow={'none'} p="20px" bg="bg" w={'100%'}>
                <HStack>
                    <Card
                        p="15px 20px"
                        shadow={'none'}
                        border={'1px'}
                        borderColor={'border'}
                        w={'50%'}
                    >
                        <Text>Balance</Text>
                        <HStack>
                            <Text>{formatAmount(balance)}</Text> <TokenSign />
                        </HStack>
                    </Card>
                    <Card
                        p="15px 20px"
                        shadow={'none'}
                        border={'1px'}
                        borderColor={'border'}
                        w={'50%'}
                    >
                        <Text>Total Transactions</Text>
                        {/* <Text>{parseInt(max) || 0}</Text> */}
                    </Card>
                </HStack>
            </Card>

            <TableContainer
                width={'100%'}
                m="0 auto"
                p="20px"
                bg="bg"
                borderRadius={'md'}
                fontSize={'16px'}
            >
                <Table bg="white" borderRadius={'sm'}>
                    <Thead>
                        <Tr
                            fontWeight={600}
                            color={'secondaryText'}
                            textTransform={'uppercase'}
                            fontSize={'12px'}
                        >
                            <Td>tx</Td>
                            <Td>Type</Td>
                            <Td>Date</Td>
                            <Td>GLDT amount</Td>
                            <Td>From</Td>
                            <Td>To</Td>
                        </Tr>
                    </Thead>
                    <Tbody fontSize={'14px'}>
                        {history?.Ok?.transactions?.map((e, i) => {
                            return (
                                <Tr key={i}>
                                    <Td>
                                        <Link href={`/transaction/${e.id}`}>{parseInt(e.id)}</Link>
                                    </Td>
                                    <Td>{e.transaction.kind}</Td>
                                    <Td>
                                        <Timestamp timestamp={parseInt(e.transaction.timestamp)} />
                                    </Td>
                                    <Td>
                                        <HStack>
                                            <Text>
                                                {formatAmount(e.transaction.transfer[0].amount)}
                                            </Text>
                                            <TokenSign />
                                        </HStack>
                                    </Td>
                                    <Td>
                                        {Principal.fromUint8Array(
                                            e.transaction.transfer[0].from.owner._arr,
                                        ).toString()}
                                    </Td>
                                    <Td>
                                        {Principal.fromUint8Array(
                                            e.transaction.transfer[0].to.owner._arr,
                                        ).toString()}
                                    </Td>
                                </Tr>
                            );
                        })}
                    </Tbody>
                </Table>
                <Pagination
                    total={100}
                    currentHistoryPage={currentPage}
                    setCurrentHistoryPage={setCurrentPage}
                    setAction={setAction}
                />
            </TableContainer>
        </VStack>
    );
};

export default AccountContent;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total, setAction }) => {
    return (
        <VStack p="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text>Page {currentHistoryPage + 1}</Text>
                <Text>{total} entries</Text>
            </Flex>
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Button
                    bg="bg"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={currentHistoryPage < 1}
                    onClick={() => {
                        setCurrentHistoryPage((prev) => prev - 1);
                        setAction(-1);
                    }}
                >
                    <ArrowBackIcon />
                </Button>
                <Button
                    bg="bg"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={total / (currentHistoryPage + 1) < 10}
                    onClick={() => {
                        setCurrentHistoryPage((prev) => prev + 1);
                        setAction(+1);
                    }}
                >
                    <ArrowForwardIcon />
                </Button>
            </Flex>
        </VStack>
    );
};
