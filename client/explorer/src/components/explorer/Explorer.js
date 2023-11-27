import {
    Box,
    Button,
    Flex,
    GridItem,
    HStack,
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
import React, { useState } from 'react';
import Timestamp from '@ui/tooltip/timeStamp';
import { Principal } from '@dfinity/principal';
import { ArrowBackIcon, ArrowForwardIcon } from '@chakra-ui/icons';
import Link from 'next/link';
import PrincipalFormat from '@ui/principal/Principal';
import TokenSign from '@ui/gldt/TokenSign';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import { formatAmount } from '@utils/misc/format';
import GridSystem from '@ui/layout/GridSystem';
import Title from '../layout/Title';
import TableTitle from '../layout/TableTitle';
import { buf2hex } from '@utils/misc/buf2hex';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';

const Explorer = () => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const [startingIndex, setStartingIndex] = useState(0);
    const { blocks, isLoading } = useBlock(startingIndex, rowsPerPage);
    const thead = ['Index', 'Date/Hour', 'Type', 'Amount', 'From', 'To'];

    return (
        <GridSystem gap={['0px', '0px', '20px']}>
            <Title title={'GLDT'} subTitle={'Explorer'} />
            <TableTitle title={'Past Transactions'} />
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
                                {!isLoading ? (
                                    blocks?.blocks?.map((e, i) => {
                                        let from = {
                                            principal: null,
                                            subaccount: null,
                                        };
                                        let to = {
                                            principal: null,
                                            subaccount: null,
                                        };
                                        let type;
                                        let tx;
                                        let memo;
                                        let amt;
                                        let fee;
                                        let ts;
                                        const labelsType = {
                                            xfer: 'Transfer',
                                            mint: 'Mint',
                                            burn: 'Burn',
                                        };
                                        e.Map.map((e, i) => {
                                            if (e[0] === 'tx') {
                                                tx = e[1].Map;
                                            }
                                            if (e[0] === 'ts') {
                                                ts = e[1].Int;
                                            }
                                            if (e[0] === 'fee') {
                                                fee = e[1].Int;
                                            }
                                        });
                                        tx.map((e) => {
                                            if (e[0] === 'memo') {
                                                memo =
                                                    e[1].Blob.length > 0
                                                        ? Principal.fromUint8Array(e[1].Blob)
                                                        : '-' || '-';
                                            }
                                            if (e[0] === 'from') {
                                                from.principal =
                                                    Principal.fromUint8Array(
                                                        e[1].Array[0].Blob,
                                                    ).toString() || '';
                                                from.subaccount = e[1].Array[1]?.Blob
                                                    ? buf2hex(e[1].Array[1]?.Blob).toString()
                                                    : undefined;
                                            }
                                            if (e[0] === 'to') {
                                                to.principal =
                                                    Principal.fromUint8Array(
                                                        e[1].Array[0].Blob,
                                                    ).toString() || '';
                                                to.subaccount = e[1].Array[1]?.Blob
                                                    ? buf2hex(e[1].Array[1]?.Blob).toString()
                                                    : undefined;
                                            }
                                            if (e[0] === 'op') {
                                                type = labelsType[e[1].Text];
                                            }
                                            if (e[0] === 'amt') {
                                                amt = e[1].Int;
                                            }
                                        });
                                        if (type === 'Mint') {
                                            from.principal = 'Minting Account';
                                            fee = '0.0000';
                                        }
                                        if (!memo) {
                                            memo = '-';
                                        }
                                        return (
                                            <Tr key={i}>
                                                <Td>
                                                    <Link
                                                        href={`/transaction/${
                                                            parseInt(blocks.chain_length) -
                                                            i +
                                                            startingIndex -
                                                            1
                                                        }`}
                                                    >
                                                        {parseInt(blocks.chain_length) -
                                                            i +
                                                            startingIndex -
                                                            1}
                                                    </Link>
                                                </Td>
                                                <Td>
                                                    <Timestamp timestamp={parseInt(ts)} />
                                                </Td>
                                                <Td>{type}</Td>
                                                <Td>
                                                    <Text fontSize={'14px'}>
                                                        {formatAmount(amt, 2)}
                                                    </Text>
                                                    <TokenSign />
                                                </Td>
                                                <Td>
                                                    <HStack>
                                                        <Link
                                                            href={
                                                                typeof from === 'string'
                                                                    ? '#'
                                                                    : `/account/${from.principal}${
                                                                          from.subaccount
                                                                              ? '?subaccount=' +
                                                                                from.subaccount
                                                                              : ''
                                                                      }`
                                                            }
                                                        >
                                                            <PrincipalFormat
                                                                principal={from.principal}
                                                                nobtn
                                                            />
                                                            {from.subaccount && (
                                                                <Box
                                                                    fontSize={'12px'}
                                                                    color={'blackAlpha.700'}
                                                                >
                                                                    <PrincipalFormat
                                                                        nobtn
                                                                        principal={from.subaccount}
                                                                    />
                                                                </Box>
                                                            )}
                                                        </Link>
                                                        <CopyPrincipal text={from.principal} />
                                                    </HStack>
                                                </Td>
                                                <Td>
                                                    <HStack>
                                                        {to.principal && (
                                                            <Link
                                                                href={
                                                                    typeof to === 'string'
                                                                        ? '#'
                                                                        : `/account/${
                                                                              to.principal
                                                                          }${
                                                                              to.subaccount
                                                                                  ? '?subaccount=' +
                                                                                    to.subaccount
                                                                                  : ''
                                                                          }`
                                                                }
                                                            >
                                                                <PrincipalFormat
                                                                    nobtn
                                                                    principal={to.principal}
                                                                />
                                                                {to.subaccount && (
                                                                    <Box
                                                                        fontSize={'12px'}
                                                                        color={'blackAlpha.700'}
                                                                    >
                                                                        <PrincipalFormat
                                                                            nobtn
                                                                            principal={
                                                                                to.subaccount
                                                                            }
                                                                        />
                                                                    </Box>
                                                                )}
                                                            </Link>
                                                        )}
                                                        <CopyPrincipal text={from.principal} />
                                                    </HStack>
                                                </Td>
                                            </Tr>
                                        );
                                    })
                                ) : (
                                    <>
                                        {Array.from({ length: rowsPerPage }).map((e, i) => (
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
                </TableContainer>
                <Pagination
                    total={parseInt(blocks.chain_length)}
                    currentHistoryPage={currentPage}
                    setCurrentHistoryPage={setCurrentPage}
                    setStartingIndex={setStartingIndex}
                />
            </GridItem>
        </GridSystem>
    );
};

export default Explorer;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total, setStartingIndex }) => {
    total = total ? total : 0;
    return (
        <VStack w={'100%'} bg="bg" px="20px" pb="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text fontSize={'14px'}>Page {currentHistoryPage + 1}</Text>
                <Text fontSize={'14px'}>{total} entries</Text>
            </Flex>
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Button
                    bg="white"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={currentHistoryPage < 1}
                    onClick={() => {
                        setCurrentHistoryPage((prev) => prev - 1);
                        setStartingIndex((prev) => prev + 10);
                    }}
                >
                    <ArrowBackIcon />
                </Button>
                <Button
                    bg="white"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={total / (currentHistoryPage + 1) < 10}
                    onClick={() => {
                        setCurrentHistoryPage((prev) => prev + 1);
                        setStartingIndex((prev) => prev - 10);
                    }}
                >
                    <ArrowForwardIcon />
                </Button>
            </Flex>
        </VStack>
    );
};
