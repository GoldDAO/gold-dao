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
import { buf2hex, stringToUint8Array } from '@utils/misc/buf2hex';
import { useRouter } from 'next/router';
import Link from 'next/link';
import { Principal } from '@dfinity/principal';
import GridSystem from '@ui/layout/GridSystem';
import Title from '../layout/Title';
import TableTitle from '../layout/TableTitle';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';

const AccountContent = ({ id, subAccount }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const router = useRouter();
    const subaccountParam = router.query.subaccount;
    const [currentSub, setCurrentSub] = useState();
    const [index, setIndex] = useState({
        last: null,
        first: null,
    });
    const [action, setAction] = useState();
    const [i, seti] = useState([[]]);
    const { history, isLoading } = useHistory(id, currentPage, router.query.subaccount, i);
    const { subaccounts } = useSubaccounts(id);
    const [balance, setBalance] = useState(0);
    const thead = ['tx', 'Type', 'Date', 'GLDT amount', 'From', 'To'];
    const oldest = history?.history?.Ok?.oldest_tx_id[0];

    useEffect(() => {
        if (history?.Ok?.transactions[history?.Ok?.transactions?.length - 1]?.id) {
            i.push(parseInt(history?.Ok?.transactions[history.Ok.transactions.length - 1].id));
        }
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
    }, [currentSub, id]);

    useEffect(() => {
        if (history) {
            setBalance(history.history.Ok.balance);
        }
    }, [history]);
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
                <HStack alignItems={'center'} justifyContent={'flex-start'}>
                    <VStack w={'100%'} alignItems={'flex-start'} justifyContent={'center'}>
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
                    <CopyPrincipal text={currentSub || router.query.subaccount || ''} />
                </HStack>
            </GridItem>
            <GridItem gridColumn={['1/12', '1/12', '1/6']}>
                <VStack alignItems={'flex-start'}>
                    <Text color={'blackAlpha.600'} fontSize={'14px'}>
                        Balance
                    </Text>
                    <HStack>
                        <Text>{Number(formatAmount(balance)).toLocaleString('en-US')}</Text>{' '}
                        <TokenSign />
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
                                {history?.history?.Ok?.transactions.length < 1 && (
                                    <Tr>
                                        <Td>No Transaction</Td>
                                    </Tr>
                                )}
                                {!isLoading ? (
                                    history?.history?.Ok?.transactions?.map((e, i) => {
                                        const from =
                                            e.transaction[e.transaction.kind][0].from || '';
                                        const to = e.transaction[e.transaction.kind][0].to || '';
                                        const id = parseInt(e.id) || '';
                                        const kind = e.transaction.kind || '';
                                        const timestamp = e.transaction.timestamp || '';
                                        const amt =
                                            e.transaction[e.transaction.kind][0].amount || '';

                                        return (
                                            <Tr key={i}>
                                                <Td>
                                                    <Link href={`/transaction/${id}`}>{id}</Link>
                                                </Td>
                                                <Td>{kind}</Td>
                                                <Td>
                                                    <Timestamp timestamp={parseInt(timestamp)} />
                                                </Td>
                                                <Td>
                                                    <HStack>
                                                        <Text fontSize={'14px'}>
                                                            {formatAmount(amt)}
                                                        </Text>
                                                        <TokenSign />
                                                    </HStack>
                                                </Td>

                                                <Td>
                                                    <HStack>
                                                        {from?.owner?._arr && from !== '' && (
                                                            <Link
                                                                href={`/account/${Principal.fromUint8Array(
                                                                    from.owner._arr,
                                                                ).toString()}${
                                                                    from.subaccount.length > 0
                                                                        ? `?subaccount=${buf2hex(
                                                                              from.subaccount[0],
                                                                          )}`
                                                                        : ''
                                                                }`}
                                                            >
                                                                <PrincipalFormat
                                                                    nobtn
                                                                    principal={Principal.fromUint8Array(
                                                                        from.owner._arr,
                                                                    ).toString()}
                                                                />
                                                                {from.subaccount.length > 0 && (
                                                                    <Box
                                                                        fontSize={'14px'}
                                                                        color={'secondaryText'}
                                                                    >
                                                                        <PrincipalFormat
                                                                            nobtn={true}
                                                                            principal={buf2hex(
                                                                                from.subaccount[0],
                                                                            )}
                                                                        />
                                                                    </Box>
                                                                )}
                                                            </Link>
                                                        )}
                                                        {from !== '' && (
                                                            <CopyPrincipal
                                                                text={Principal.fromUint8Array(
                                                                    from.owner._arr,
                                                                )}
                                                            />
                                                        )}
                                                    </HStack>
                                                </Td>
                                                <Td>
                                                    <HStack>
                                                        {to?.owner?._arr && to !== '' && (
                                                            <Link
                                                                href={`/account/${Principal.fromUint8Array(
                                                                    to.owner._arr,
                                                                ).toString()}${
                                                                    to.subaccount.length > 0
                                                                        ? `?subaccount=${buf2hex(
                                                                              to.subaccount[0],
                                                                          )}`
                                                                        : ''
                                                                }`}
                                                            >
                                                                <PrincipalFormat
                                                                    nobtn
                                                                    principal={Principal.fromUint8Array(
                                                                        to.owner._arr,
                                                                    ).toString()}
                                                                />
                                                                {to.subaccount.length > 0 && (
                                                                    <Box
                                                                        fontSize={'14px'}
                                                                        color={'secondaryText'}
                                                                    >
                                                                        <PrincipalFormat
                                                                            nobtn={true}
                                                                            principal={buf2hex(
                                                                                to.subaccount[0],
                                                                            )}
                                                                        />
                                                                    </Box>
                                                                )}
                                                            </Link>
                                                        )}
                                                        {to !== '' && (
                                                            <CopyPrincipal
                                                                text={Principal.fromUint8Array(
                                                                    to.owner._arr,
                                                                )}
                                                            />
                                                        )}
                                                    </HStack>
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
                    {history?.history?.Ok?.transactions.length > 0 && (
                        <Pagination
                            currentHistoryPage={currentPage}
                            setCurrentHistoryPage={setCurrentPage}
                            setAction={setAction}
                            oldest={oldest}
                            last={
                                history?.history?.Ok?.transactions[
                                    history?.history?.Ok?.transactions.length - 1
                                ]
                            }
                        />
                    )}
                </TableContainer>
            </GridItem>
        </GridSystem>
    );
};

export default AccountContent;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, oldest, setAction, last }) => {
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
                    isDisabled={last.id === oldest}
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
