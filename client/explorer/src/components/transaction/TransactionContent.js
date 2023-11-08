import { buf2hex } from '@utils/misc/buf2hex';
import {
    Card,
    GridItem,
    Table,
    Tbody,
    Td,
    Thead,
    Tr,
    Text,
    Heading,
    HStack,
    VStack,
} from '@chakra-ui/react';
import Timestamp from '@ui/tooltip/timeStamp';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import Link from 'next/link';
import React, { useState } from 'react';
import PrincipalFormat from '@ui/principal/Principal';
import { Principal } from '@dfinity/principal';
import GridSystem from '@ui/layout/GridSystem';
import TokenSign from '@ui/gldt/TokenSign';

const TransactionContent = ({ id }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const { blocks, isLoading } = useBlock(0, 0, id);

    if (blocks.blocks) {
        const from = blocks.blocks[0].Map[2][1].Map[2][1].Text
            ? 'Minting account'
            : blocks.blocks[0].Map[2][1].Map[2][1].Array[0].Blob;
        let to;
        blocks.blocks[0].Map[2][1].Map.map((e, i) => {
            if (e[0] === 'to') {
                to = e[1].Array[0].Blob;
            }
        });

        return (
            <GridSystem>
                <GridItem gridColumn={['2/8']} py="40px">
                    <Heading as="h1" variant={'h1'}>
                        Transaction
                    </Heading>
                    <Heading as="h2" variant={'h2'}>
                        {id}
                    </Heading>
                </GridItem>
                <GridItem gridColumn={['8/11']} py="40px">
                    <HStack justifyContent={'space-between'}>
                        <Text fontSize={'16px'}>Amount: </Text>
                        <HStack>
                            <Text as="h3" variant={'h2'}>
                                {parseInt(blocks.blocks[0].Map[2][1].Map[0][1].Int)}
                            </Text>
                            <TokenSign />
                        </HStack>
                    </HStack>
                    <HStack justifyContent={'space-between'}>
                        <Text fontSize={'16px'}>Fees: </Text>
                        <HStack>
                            <Text>{parseInt(blocks.blocks[0].Map[2][1].Map[1][1].Int) || 0}</Text>
                            <TokenSign />
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colStart={['1', '1', '2']} colSpan={[3]}>
                    <VStack
                        alignItems={'flex-start'}
                        fontSize={'20px'}
                        pt="20px"
                        borderTop={'1px'}
                        borderColor={'secondaryText'}
                    >
                        <Text fontWeight={500} fontSize={'16px'}>
                            Date/Hour
                        </Text>
                        <Timestamp timestamp={parseInt(blocks.blocks[0].Map[1][1].Int)} />
                    </VStack>
                </GridItem>
                <GridItem colSpan={[3]}>
                    <VStack
                        alignItems={'flex-start'}
                        fontSize={'20px'}
                        pt="20px"
                        borderTop={'1px'}
                        borderColor={'secondaryText'}
                    >
                        <Text fontWeight={500} fontSize={'16px'}>
                            from
                        </Text>
                        <Link
                            href={
                                typeof from === 'string'
                                    ? '#'
                                    : `/account/${Principal.fromUint8Array(from).toString()}`
                            }
                        >
                            {typeof from === 'string' ? (
                                from
                            ) : (
                                <PrincipalFormat
                                    principal={Principal.fromUint8Array(from).toString()}
                                />
                            )}
                        </Link>
                    </VStack>
                </GridItem>
                <GridItem colSpan={[3]}>
                    <VStack
                        alignItems={'flex-start'}
                        fontSize={'20px'}
                        pt="20px"
                        borderTop={'1px'}
                        borderColor={'secondaryText'}
                    >
                        <Text fontWeight={500} fontSize={'16px'}>
                            To
                        </Text>
                        <Link href={`/account/${Principal.fromUint8Array(to).toString()}`}>
                            <PrincipalFormat principal={Principal.fromUint8Array(to).toString()} />
                        </Link>
                    </VStack>
                </GridItem>
                {/* <GridItem gridColumn={['1/13', '1/13', '2/13', '2/13', '2/13']}>
                    <Table bg="white" borderRadius={'sm'}>
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
                            <Td>{id}</Td>
                            <Td>
                                <Timestamp timestamp={parseInt(blocks.blocks[0].Map[1][1].Int)} />
                            </Td>
                            <Td>{parseInt(blocks.blocks[0].Map[2][1].Map[0][1].Int)}</Td>
                            <Td>{parseInt(blocks.blocks[0].Map[2][1].Map[1][1].Int) || 0}</Td>
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
                                            principal={Principal.fromUint8Array(from).toString()}
                                        />
                                    )}
                                </Link>
                            </Td>
                            <Td>
                                <Link href={`/account/${Principal.fromUint8Array(to).toString()}`}>
                                    <PrincipalFormat
                                        principal={Principal.fromUint8Array(to).toString()}
                                    />
                                </Link>
                            </Td>
                        </Tbody>
                    </Table>
                </GridItem> */}
            </GridSystem>
        );
    }
};

export default TransactionContent;
