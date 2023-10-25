import { buf2hex } from '@/utils/buf2hex';
import { Card, Table, Tbody, Td, Thead, Tr } from '@chakra-ui/react';
import Timestamp from '@ui/tooltip/timeStamp';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import Link from 'next/link';
import React, { useState } from 'react';
import PrincipalFormat from '../Principal';
import { Principal } from '@dfinity/principal';

const TransactionContent = ({ id }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const { blocks, isLoading } = useBlock(0, 0, id);

    console.log('isLoading', isLoading);

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
                    </Td>
                    <Td>
                        <Link href={`/account/${Principal.fromUint8Array(to).toString()}`}>
                            <PrincipalFormat principal={Principal.fromUint8Array(to).toString()} />
                        </Link>
                    </Td>
                </Tbody>
            </Table>
        );
    }
};

export default TransactionContent;
