import {
    Box,
    Button,
    Card,
    Flex,
    GridItem,
    HStack,
    Heading,
    Select,
    Skeleton,
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
import Title from '../layout/Title';
import TableTitle from '../layout/TableTitle';

const AccountContent = ({ id, subAccount }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [currentSub, setCurrentSub] = useState();
    const [index, setIndex] = useState({
        last: null,
        first: null,
    });
    const [action, setAction] = useState();
    const [i, seti] = useState([[]]);
    const { history, isLoading } = useHistory(id, currentPage, currentSub, i);
    const { subaccounts } = useSubaccounts(id);
    const { balance } = useBalance(id, currentSub ? currentSub : subAccount);
    const router = useRouter();

    useEffect(() => {
        if (history?.Ok?.transactions[history?.Ok?.transactions?.length - 1]?.id) {
            i.push(parseInt(history?.Ok?.transactions[history.Ok.transactions.length - 1].id));
        }
    }, [history]);

    useEffect(() => {
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
    const thead = ['tx', 'Type', 'Date', 'GLDT amount', 'From', 'To'];

    return (
        <GridSystem gap={['0px', '0px', '20px']}>
            <Title title="GLDT" subTitle={'Account'} />
            <GridItem
                gridColumn={['1/12', '1/12', '1/12']}
                alignSelf={['flex-start', 'flex-start', 'flex-end']}
            >
                <VStack alignItems={'flex-start'}>
                    <Text color={'blackAlpha.600'} fontSize={'14px'}>
                        Principal-ID
                    </Text>
                    <HStack>
                        <PrincipalFormat principal={id} full />
                    </HStack>
                </VStack>
            </GridItem>
            <GridItem
                gridColumn={['1/12', '1/12', '1/6']}
                alignSelf={['flex-start', 'flex-start', 'flex-end']}
            >
                <VStack alignItems={'flex-start'}>
                    <Text color={'blackAlpha.600'} fontSize={'14px'}>
                        Subaccounts
                    </Text>
                    <Select
                        size="md"
                        width={['100%', '100%']}
                        onChange={toggleChange}
                        placeholder={subAccount}
                        value={currentSub}
                    >
                        {subaccounts.map((e, i) => (
                            <option key={i} value={buf2hex(e)}>
                                {parseInt(buf2hex(e)) === 0 ? 'Default (0)' : buf2hex(e)}
                            </option>
                        ))}
                    </Select>
                </VStack>
            </GridItem>
            <GridItem gridColumn={['1/12', '1/12', '1/6']}>
                <VStack alignItems={'flex-start'}>
                    <Text color={'blackAlpha.600'} fontSize={'14px'}>
                        Balance
                    </Text>
                    <HStack>
                        <Text>{formatAmount(balance)}</Text> <TokenSign />
                    </HStack>
                </VStack>
            </GridItem>

            <TableTitle title={'History'} />
            <GridItem gridColumn={['1/13', '1/13', '1/13']}>
                <TableContainer width={'100%'} m="0 auto" p="20px" bg="bg" borderRadius={'md'}>
                    <Box w={'100%'} overflow={'scroll'}>
                        <Table
                            bg="white"
                            borderRadius={'sm'}
                            overflow-x="auto"
                            w={'100%'}
                            white-space="nowrap"
                        >
                            <Thead display="table-header-group">
                                <Tr
                                    fontWeight={600}
                                    color={'secondaryText'}
                                    textTransform={'uppercase'}
                                    fontSize={'12px'}
                                >
                                    {thead.map((e, i) => (
                                        <Td key={i}>{e}</Td>
                                    ))}
                                </Tr>
                            </Thead>
                            <Tbody fontSize={'14px'}>
                                {history?.Ok?.transactions.length < 1 && (
                                    <Tr>
                                        <Td>No Transaction</Td>
                                    </Tr>
                                )}
                                {!isLoading ? (
                                    history?.Ok?.transactions?.map((e, i) => {
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
                                                        timestamp={parseInt(
                                                            e.transaction.timestamp,
                                                        )}
                                                    />
                                                </Td>
                                                <Td>
                                                    <HStack>
                                                        <Text fontSize={'14px'}>
                                                            {formatAmount(
                                                                e.transaction.transfer[0].amount,
                                                            )}
                                                        </Text>
                                                        <TokenSign />
                                                    </HStack>
                                                </Td>
                                                <Td>
                                                    <Link
                                                        href={`/account/${Principal.fromUint8Array(
                                                            e.transaction.transfer[0].from.owner
                                                                ._arr,
                                                        ).toString()}`}
                                                    >
                                                        <PrincipalFormat
                                                            principal={Principal.fromUint8Array(
                                                                e.transaction.transfer[0].from.owner
                                                                    ._arr,
                                                            ).toString()}
                                                        />
                                                        {e.transaction.transfer[0].from.subaccount
                                                            .length > 0 && (
                                                            <Box
                                                                fontSize={'14px'}
                                                                mt="-10px"
                                                                color={'secondaryText'}
                                                            >
                                                                <PrincipalFormat
                                                                    nobtn={true}
                                                                    principal={Principal.fromUint8Array(
                                                                        e.transaction.transfer[0]
                                                                            .from.subaccount,
                                                                    ).toString()}
                                                                />
                                                            </Box>
                                                        )}
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
                                                                e.transaction.transfer[0].to.owner
                                                                    ._arr,
                                                            ).toString()}
                                                        />
                                                    </Link>
                                                </Td>
                                            </Tr>
                                        );
                                    })
                                ) : (
                                    <>
                                        {Array.from({ length: 10 }).map((e, i) => (
                                            <Tr width={'100%'} key={i} borderRadius={'20px'}>
                                                {thead.map((e, i) => (
                                                    <Td key={i} p="0">
                                                        <Skeleton
                                                            height={'73px'}
                                                            w={'100%'}
                                                            startColor="blackAlpha.100"
                                                            endColor="blackAlpha.300"
                                                        />
                                                    </Td>
                                                ))}
                                            </Tr>
                                        ))}
                                    </>
                                )}
                            </Tbody>
                        </Table>
                    </Box>
                    {history?.Ok?.transactions.length > 0 && (
                        <Pagination
                            currentHistoryPage={currentPage}
                            setCurrentHistoryPage={setCurrentPage}
                            setAction={setAction}
                        />
                    )}
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
