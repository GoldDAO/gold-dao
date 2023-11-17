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
    Box,
} from '@chakra-ui/react';
import Timestamp from '@ui/tooltip/timeStamp';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import Link from 'next/link';
import React, { useState } from 'react';
import PrincipalFormat from '@ui/principal/Principal';
import { Principal } from '@dfinity/principal';
import GridSystem from '@ui/layout/GridSystem';
import TokenSign from '@ui/gldt/TokenSign';
import Title from '../layout/Title';
import { formatAmount } from '@utils/misc/format';

const TransactionContent = ({ id }) => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const { blocks, isLoading } = useBlock(0, 0, id);
    if (blocks.blocks) {
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
        let fee;
        let amt;
        let ts;
        const labelsType = {
            xfer: 'Transfer',
            mint: 'Mint',
            burn: 'Burn',
        };
        blocks.blocks[0].Map.map((e, i) => {
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
                console.log('e[1].Blob', e[1].Blob);
                memo = e[1].Blob.length > 0 ? Principal.fromUint8Array(e[1].Blob) : '-' || '-';
            }
            if (e[0] === 'from') {
                from.principal = Principal.fromUint8Array(e[1].Array[0].Blob).toString() || '';
                from.subaccount = Principal.fromUint8Array(e[1].Array[1]?.Blob) || '';
            }
            if (e[0] === 'to') {
                to.principal = Principal.fromUint8Array(e[1].Array[0].Blob).toString() || '';
                to.subaccount = Principal.fromUint8Array(e[1].Array[1]?.Blob) || '';
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
            <GridSystem gap={'40px'}>
                <Title title={'GLDT '} subTitle={'Transaction'} />
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Block Index:
                        </Text>
                        <HStack>
                            <Text>{id}</Text>
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Type:
                        </Text>
                        <HStack>
                            <Text>{type}</Text>
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Amount:
                        </Text>
                        <HStack>
                            <Text>
                                {formatAmount(
                                    blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1]
                                        .Map[0][1].Int,
                                )}
                            </Text>
                            <TokenSign />
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            fee:
                        </Text>
                        <HStack>
                            <Text>{formatAmount(fee, 4)}</Text>
                            <TokenSign />
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-start'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Date/Hour
                        </Text>
                        <HStack>
                            <Timestamp timestamp={parseInt(ts)} />
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            From
                        </Text>
                        <HStack>
                            <Link
                                href={typeof from === 'string' ? '#' : `/account/${from.principal}`}
                            >
                                <PrincipalFormat full principal={from.principal} />
                            </Link>
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-end'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            To
                        </Text>
                        <HStack>
                            <Link href={typeof to === 'string' ? '#' : `/account/${to.principal}`}>
                                <PrincipalFormat full principal={to.principal} />
                            </Link>
                        </HStack>
                    </HStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 12]}>
                    <HStack alignItems={'flex-start'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Memo
                        </Text>
                        <HStack>
                            {memo && memo.length > 5 ? (
                                <PrincipalFormat full principal={memo} />
                            ) : (
                                <Text>{memo}</Text>
                            )}
                        </HStack>
                    </HStack>
                </GridItem>
            </GridSystem>
        );
    }
};

export default TransactionContent;
