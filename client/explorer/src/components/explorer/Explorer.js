import {
    Button,
    Flex,
    GridItem,
    Heading,
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
import { ArrowBackIcon, ArrowForwardIcon, Search2Icon } from '@chakra-ui/icons';
import Link from 'next/link';
import PrincipalFormat from '@ui/principal/Principal';
import TokenSign from '@ui/gldt/TokenSign';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import { formatAmount } from '@utils/misc/format';
import GridSystem from '@ui/layout/GridSystem';
import Title from '../layout/Title';
import TableTitle from '../layout/TableTitle';

const Explorer = () => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const [startingIndex, setStartingIndex] = useState(0);
    const { blocks } = useBlock(startingIndex, rowsPerPage);

    return (
        <GridSystem gap={['0px', '0px', '60px']}>
            <Title title={'GLDT'} subTitle={'Explorer'} />
            <TableTitle title={'Past Transactions'} />
            <GridItem gridColumn={['1/13', '1/13', '2/12']}>
                <TableContainer width={'100%'} m="0 auto" p="20px" bg="bg" borderRadius={'md'}>
                    <Table
                        bg="white"
                        borderRadius={'sm'}
                        w={'100%'}
                        display={'block'}
                        overflow={'scroll'}
                    >
                        <Thead>
                            <Tr
                                fontWeight={600}
                                color={'secondaryText'}
                                textTransform={'uppercase'}
                                fontSize={'12px'}
                            >
                                <Td>Index</Td>
                                <Td>Date/Hour</Td>
                                <Td>Amount</Td>
                                <Td>Fees</Td>
                                <Td>From</Td>
                                <Td>To</Td>
                            </Tr>
                        </Thead>
                        <Tbody fontSize={'14px'}>
                            {blocks?.blocks?.map((e, i) => {
                                const from = e.Map[2][1].Map[2][1].Text
                                    ? 'Minting account'
                                    : e.Map[2][1].Map[2][1].Array[0].Blob;
                                let to;
                                e.Map[2][1].Map.map((e, i) => {
                                    if (e[0] === 'to') {
                                        to = e[1].Array[0].Blob;
                                    }
                                });
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
                                            <Timestamp timestamp={parseInt(e.Map[1][1].Int)} />
                                        </Td>
                                        <Td>
                                            <Text fontSize={'14px'}>
                                                {formatAmount(parseInt(e.Map[2][1].Map[0][1].Int))}
                                            </Text>
                                            <TokenSign />
                                        </Td>
                                        <Td>
                                            <Text fontSize={'14px'}>
                                                {formatAmount(
                                                    parseInt(e.Map[2][1].Map[1][1].Int || 0),
                                                )}
                                            </Text>
                                            <TokenSign />
                                        </Td>
                                        <Td>
                                            <Link
                                                href={
                                                    typeof from === 'string'
                                                        ? '#'
                                                        : `/account/${Principal.fromUint8Array(
                                                              from,
                                                          ).toString()}`
                                                }
                                            >
                                                {typeof from === 'string' ? (
                                                    from
                                                ) : (
                                                    <PrincipalFormat
                                                        principal={Principal.fromUint8Array(
                                                            from,
                                                        ).toString()}
                                                    />
                                                )}
                                            </Link>
                                        </Td>
                                        <Td>
                                            <Link
                                                href={`/account/${Principal.fromUint8Array(
                                                    to,
                                                ).toString()}`}
                                            >
                                                <PrincipalFormat
                                                    principal={Principal.fromUint8Array(
                                                        to,
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
                        total={parseInt(blocks.chain_length)}
                        currentHistoryPage={currentPage}
                        setCurrentHistoryPage={setCurrentPage}
                        setStartingIndex={setStartingIndex}
                    />
                </TableContainer>
            </GridItem>
        </GridSystem>
    );
};

export default Explorer;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total, setStartingIndex }) => {
    total = total ? total : 0;
    return (
        <VStack pt="20px" w={'100%'}>
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
