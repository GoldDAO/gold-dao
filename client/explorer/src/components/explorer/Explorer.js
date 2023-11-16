import {
    Box,
    Button,
    Flex,
    GridItem,
    Heading,
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
                                        let from;
                                        let to;
                                        let type;
                                        const labelsType = {
                                            xfer: 'Transfer',
                                            mint: 'Mint',
                                            burn: 'Burn',
                                        };

                                        let fromI;
                                        e?.Map[e.Map.length - 1][1]?.Map.map((e, i) => {
                                            console.log('e[0]', e[0]);
                                            if (e[0] === 'from') {
                                                fromI = i;
                                            }
                                        });
                                        from = !e?.Map[e.Map.length - 1][1]?.Map[fromI]
                                            ? 'Minting account'
                                            : {
                                                  principal:
                                                      e.Map[e.Map.length - 1][1].Map[fromI]?.[1]
                                                          .Array[0].Blob,
                                                  subaccount:
                                                      e.Map[e.Map.length - 1][1].Map[fromI]?.[1]
                                                          .Array[1]?.Blob,
                                              };
                                        e.Map[e.Map.length - 1][1].Map.map((el, i) => {
                                            if (el[0] === 'to') {
                                                to = {
                                                    principal: el[1].Array[0].Blob,
                                                    subaccount: el[1].Array[1]?.Blob,
                                                };
                                            }
                                        });

                                        for (
                                            let i = 0;
                                            i < e.Map[e.Map.length - 1][1].Map.length - 1;
                                            i++
                                        ) {
                                            if (e.Map[e.Map.length - 1][1].Map[i][0] === 'op') {
                                                type =
                                                    labelsType[
                                                        e.Map[e.Map.length - 1][1].Map[i][1].Text
                                                    ];
                                            }
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
                                                    <Timestamp
                                                        timestamp={parseInt(
                                                            e.Map[e.Map.length - 2][1].Int,
                                                        )}
                                                    />
                                                </Td>
                                                <Td>{type}</Td>
                                                <Td>
                                                    <Text fontSize={'14px'}>
                                                        {formatAmount(
                                                            e.Map[e.Map.length - 1][1].Map[0][1]
                                                                .Int,
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
                                                                      from.principal,
                                                                  ).toString()}`
                                                        }
                                                    >
                                                        {typeof from === 'string' ? (
                                                            from
                                                        ) : (
                                                            <Box fontSize={'16px'}>
                                                                <PrincipalFormat
                                                                    principal={Principal.fromUint8Array(
                                                                        from.principal,
                                                                    ).toString()}
                                                                />
                                                                {from.subaccount && (
                                                                    <Box
                                                                        fontSize={'14px'}
                                                                        mt="-10px"
                                                                        color={'secondaryText'}
                                                                    >
                                                                        <PrincipalFormat
                                                                            nobtn={true}
                                                                            principal={Principal.fromUint8Array(
                                                                                from.subaccount,
                                                                            ).toString()}
                                                                        />
                                                                    </Box>
                                                                )}
                                                            </Box>
                                                        )}
                                                    </Link>
                                                </Td>
                                                <Td>
                                                    {to.principal && (
                                                        <Link
                                                            href={
                                                                typeof to === 'string'
                                                                    ? '#'
                                                                    : `/account/${Principal.fromUint8Array(
                                                                          to.principal,
                                                                      ).toString()}`
                                                            }
                                                        >
                                                            <Box fontSize={'16px'}>
                                                                <PrincipalFormat
                                                                    principal={Principal.fromUint8Array(
                                                                        to.principal,
                                                                    ).toString()}
                                                                />
                                                                {to.subaccount && (
                                                                    <Box
                                                                        fontSize={'14px'}
                                                                        mt="-10px"
                                                                        color={'secondaryText'}
                                                                    >
                                                                        <PrincipalFormat
                                                                            nobtn={true}
                                                                            principal={Principal.fromUint8Array(
                                                                                to.subaccount,
                                                                            ).toString()}
                                                                        />
                                                                    </Box>
                                                                )}
                                                            </Box>
                                                        </Link>
                                                    )}
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
