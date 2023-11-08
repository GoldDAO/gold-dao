import {
    Box,
    Button,
    Card,
    Flex,
    GridItem,
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
import { useBalance } from '@utils/hooks/ledgerIndexer/useBalance';
import { useHistory } from '@utils/hooks/ledgerIndexer/useHistory';
import { formatAmount } from '@utils/misc/format';
import PrincipalFormat from '@ui/principal/Principal';
import Timestamp from '@ui/tooltip/timeStamp';
import TokenSign from '@ui/gldt/TokenSign';
import { ArrowBackIcon, ArrowForwardIcon } from '@chakra-ui/icons';
import AccountTitle from './AccountTitle';
import { buf2hex } from '@utils/misc/buf2hex';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { Principal } from '@dfinity/principal';
import GridSystem from '@ui/layout/GridSystem';

const AccountContent = ({ id, subAccount }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [currentSub, setCurrentSub] = useState();
    const [index, setIndex] = useState({
        last: null,
        first: null,
    });
    const [last, setlast] = useState([]);
    const [action, setAction] = useState();
    const [i, seti] = useState([[]]);
    const { history } = useHistory(id, currentPage, currentSub, i);
    const { subaccounts } = useSubaccounts(id);
    const { balance } = useBalance(id, currentSub ? currentSub : subAccount);
    const router = useRouter();

    useEffect(() => {
        if (history?.Ok?.transactions[history?.Ok?.transactions?.length - 1]?.id) {
            i.push(parseInt(history?.Ok?.transactions[history.Ok.transactions.length - 1].id));
        }
    }, [history]);

    useEffect(() => {
        console.log('history', history);
        if (history?.Ok?.transactions.length > 0) {
            setIndex({
                last: history?.Ok?.transactions[history.Ok.transactions.length - 1].id,
                first: history?.Ok?.transactions[0].id,
            });
        }
    }, [history]);

    const toggleChange = (e) => {
        setCurrentSub(e.target.value);
    };

    useEffect(() => {
        if (currentSub) {
            router.push(`/account/${id}?subaccount=${currentSub}`);
        }
    }, [currentSub, id, router]);

    return (
        <GridSystem>
            <GridItem gridColumn={['1/12', '1/12', '2/12, 2/6']} py={['0px', '0px', '40px']}>
                <Heading as="h1" variant={'h1'}>
                    Account
                </Heading>
                <Heading as="h2" variant={'h2'}>
                    <PrincipalFormat principal={id} />
                </Heading>
            </GridItem>
            <GridItem
                colSpan={[12, 12, 3, 2]}
                colStart={[1, 1, 3]}
                alignSelf={['flex-start', 'flex-start', 'flex-end']}
                py={['0px', '0px', '40px']}
            >
                <AccountTitle
                    data={{
                        label: 'AccountID',
                        id: '20e43f0bd4f09346ed0bfd7006ed3a0df564c1a1e6eb483f8315d592f872e98f',
                    }}
                />
            </GridItem>
            <GridItem
                colSpan={[12, 12, 3, 2]}
                alignSelf={['flex-start', 'flex-start', 'flex-end']}
                py={['0px', '0px', '40px']}
            >
                <Select
                    size="md"
                    onChange={toggleChange}
                    placeholder={subAccount}
                    value={currentSub}
                >
                    {subaccounts.map((e, i) => (
                        <option key={i} value={buf2hex(e)}>
                            {buf2hex(e)}
                        </option>
                    ))}
                </Select>
            </GridItem>
            <GridItem
                alignSelf={['flex-start', 'flex-start', 'flex-end']}
                py={['0px', '0px', '40px']}
                colSpan={[12, 12, 3, 2]}
            >
                <HStack>
                    <Text>{formatAmount(balance)}</Text> <TokenSign />
                </HStack>
            </GridItem>
            <GridItem gridColumn={['1/13', '1/13', '1/2', '1/2', '1/2']}>
                <Heading
                    fontWeight={300}
                    as="h3"
                    fontSize={'16px'}
                    textAlign={'right'}
                    w={'100%'}
                    borderBottom="1px"
                    borderBottomColor={'secondaryText'}
                >
                    History
                </Heading>
            </GridItem>
            <GridItem gridColumn={['1/12', '1/12', '2/12']}>
                <TableContainer width={'100%'} m="0 auto" p="20px" bg="bg" borderRadius={'md'}>
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
                                            <Link href={`/transaction/${e.id}`}>
                                                {parseInt(e.id)}
                                            </Link>
                                        </Td>
                                        <Td>{e.transaction.kind}</Td>
                                        <Td>
                                            <Timestamp
                                                timestamp={parseInt(e.transaction.timestamp)}
                                            />
                                        </Td>
                                        <Td>
                                            <HStack>
                                                <Text fontSize={'14px'}>
                                                    {formatAmount(e.transaction.transfer[0].amount)}
                                                </Text>
                                                <TokenSign />
                                            </HStack>
                                        </Td>
                                        <Td>
                                            <Link
                                                href={`/account/${Principal.fromUint8Array(
                                                    e.transaction.transfer[0].from.owner._arr,
                                                ).toString()}`}
                                            >
                                                <PrincipalFormat
                                                    principal={Principal.fromUint8Array(
                                                        e.transaction.transfer[0].from.owner._arr,
                                                    ).toString()}
                                                />
                                            </Link>
                                        </Td>
                                        <Td>
                                            <Link
                                                href={`/account/${Principal.fromUint8Array(
                                                    e.transaction.transfer[0].to.owner._arr,
                                                ).toString()}`}
                                            >
                                                <PrincipalFormat
                                                    principal={Principal.fromUint8Array(
                                                        e.transaction.transfer[0].to.owner._arr,
                                                    ).toString()}
                                                />
                                            </Link>
                                        </Td>
                                    </Tr>
                                );
                            })}
                        </Tbody>
                    </Table>
                    <Pagination
                        currentHistoryPage={currentPage}
                        setCurrentHistoryPage={setCurrentPage}
                        setAction={setAction}
                    />
                </TableContainer>
            </GridItem>
        </GridSystem>
    );
};

export default AccountContent;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total, setAction }) => {
    return (
        <VStack p="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text fontSize={'14px'}>Page {currentHistoryPage + 1}</Text>
                {/* <Text>{total} entries</Text> */}
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
                    // isDisabled={total / (currentHistoryPage + 1) < 10}
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
