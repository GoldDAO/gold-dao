import { buf2hex } from '@utils/misc/buf2hex';
import { GridItem, Table, Tr, Td, TableContainer, HStack, Text, Tbody } from '@chakra-ui/react';
import Timestamp from '@ui/tooltip/timeStamp';
import { useBlock } from '@utils/hooks/ledgerIndexer/useBlock';
import Link from 'next/link';
import React from 'react';
import PrincipalFormat from '@ui/principal/Principal';
import { Principal } from '@dfinity/principal';
import GridSystem from '@ui/layout/GridSystem';
import TokenSign from '@ui/gldt/TokenSign';
import Title from '../layout/Title';
import { formatAmount } from '@utils/misc/format';
import { formatNumber } from '@/utils/misc';

const TransactionContent = ({ id }) => {
    const { blocks, isLoading } = useBlock(0, 0, id);
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

    if (blocks.blocks) {
        const labelsType = {
            xfer: 'Transfer',
            mint: 'Mint',
            burn: 'Burn',
        };
        blocks.blocks[0].Map.map((e, i) => {
            if (e[0] === 'ts') {
                ts = e[1].Int;
            }
            switch (e[0]) {
                case 'tx':
                    tx = e[1].Map;
                    break;
                case 'ts':
                    ts = e[1].Int;
                    break;
            }
        });
        tx.map((e) => {
            switch (e[0]) {
                case 'memo':
                    memo = e[1].Blob.length > 0 ? buf2hex(e[1].Blob) : '0' || '0';
                    break;
                case 'from':
                    from.principal = Principal.fromUint8Array(e[1].Array[0].Blob).toString() || '';
                    from.subaccount = buf2hex(e[1].Array[1]?.Blob) || '';
                    break;
                case 'to':
                    to.principal = Principal.fromUint8Array(e[1].Array[0].Blob).toString() || '';
                    to.subaccount = buf2hex(e[1].Array[1]?.Blob) || '';
                    break;
                case 'op':
                    type = labelsType[e[1].Text] ? labelsType[e[1].Text] : '';
                    break;
                case 'amt':
                    amt = e[1].Int ? e[1].Int : '';
                    break;
                case 'fee':
                    fee = e[1].Int ? e[1].Int : 0;
                    break;
            }
        });
        if (type === 'Mint') {
            from.principal = 'Minting Account';
            fee = '0.0000';
        }
        if (!memo) {
            memo = '0';
        }
        if (!fee) {
            fee = 0;
        }
        const data = [
            { label: 'Block Index', value: id },
            { label: 'Type', value: type },
            {
                label: 'Amount',
                value: (
                    <HStack>
                        <Text>{formatNumber(formatAmount(amt, 4))}</Text>
                        <TokenSign />
                    </HStack>
                ),
            },
            {
                label: 'Fee',
                value: (
                    <HStack>
                        <Text>{formatNumber(formatAmount(fee, 4))}</Text>
                        <TokenSign />
                    </HStack>
                ),
            },
            { label: 'Date/Hour', value: <Timestamp timestamp={parseInt(ts)} /> },
            {
                label: 'from',
                value: (
                    <Link href={typeof from === 'string' ? '#' : `/account/${from.principal}`}>
                        <PrincipalFormat full principal={from.principal} />
                    </Link>
                ),
            },
            {
                label: 'to',
                value: (
                    <Link href={typeof to === 'string' ? '#' : `/account/${to.principal}`}>
                        <PrincipalFormat full principal={to.principal} />
                    </Link>
                ),
            },
            { label: 'memo', value: memo },
        ];

        return (
            <GridSystem gap={'40px'}>
                <Title title={'GLDT '} subTitle={'Transaction'} />
                <GridItem colSpan={[12, 12, 12]}>
                    <TableContainer>
                        <Table>
                            <Tbody>
                                {data.map((e, i) => {
                                    return (
                                        <Tr key={i}>
                                            <Td> {e.label}:</Td>
                                            <Td>{e.value}</Td>
                                        </Tr>
                                    );
                                })}
                            </Tbody>
                        </Table>
                    </TableContainer>
                </GridItem>
            </GridSystem>
        );
    }
};

export default TransactionContent;
