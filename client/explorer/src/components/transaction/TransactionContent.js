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
        // const from = blocks.blocks[0].Map[2][1].Map[2][1].Text
        //     ? 'Minting account'
        //     : blocks.blocks[0].Map[2][1].Map[2][1].Array[0].Blob;
        // blocks.blocks[0].Map[2][1].Map.map((e, i) => {
        //     if (e[0] === 'to') {
        //         to = e[1].Array[0].Blob;
        //     }
        // });
        let from;
        let to;
        let type;
        const labelsType = {
            xfer: 'Transfer',
            mint: 'Mint',
            burn: 'Burn',
        };

        let fromI;
        blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1]?.Map.map((e, i) => {
            console.log('e[0]', e[0]);
            if (e[0] === 'from') {
                fromI = i;
            }
        });
        from = !blocks.blocks[0]?.Map[blocks.blocks[0].Map.length - 1][1]?.Map[fromI]
            ? 'Minting account'
            : {
                  principal:
                      blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map[fromI]?.[1]
                          .Array[0].Blob,
                  subaccount:
                      blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map[fromI]?.[1]
                          .Array[1]?.Blob,
              };
        blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map.map((el, i) => {
            if (el[0] === 'to') {
                to = {
                    principal: el[1].Array[0].Blob,
                    subaccount: el[1].Array[1]?.Blob,
                };
            }
        });

        for (
            let i = 0;
            i < blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map.length - 1;
            i++
        ) {
            if (blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map[i][0] === 'op') {
                type =
                    labelsType[
                        blocks.blocks[0].Map[blocks.blocks[0].Map.length - 1][1].Map[i][1].Text
                    ];
            }
        }
        console.log('to', to);
        return (
            <GridSystem gap={'40px'}>
                <Title title={'Transction'} subTitle={id} />
                <GridItem colSpan={[12, 12, 4]}>
                    <VStack alignItems={'flex-start'}>
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
                    </VStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 4]}>
                    <VStack alignItems={'flex-start'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            Date/Hour
                        </Text>
                        <HStack>
                            <Timestamp
                                timestamp={parseInt(
                                    blocks.blocks[0].Map[blocks.blocks[0].Map.length - 2][1].Int,
                                )}
                            />
                        </HStack>
                    </VStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 4]}>
                    <VStack alignItems={'flex-start'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            From
                        </Text>
                        <HStack>
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
                                    <PrincipalFormat
                                        full
                                        principal={Principal.fromUint8Array(
                                            from.principal,
                                        ).toString()}
                                    />
                                )}
                            </Link>
                        </HStack>
                    </VStack>
                </GridItem>
                <GridItem colSpan={[12, 12, 4]}>
                    <VStack alignItems={'flex-start'}>
                        <Text color={'blackAlpha.600'} fontSize={'14px'}>
                            To
                        </Text>
                        <HStack>
                            <Link href={`/account/${Principal.fromUint8Array(to).toString()}`}>
                                <PrincipalFormat
                                    full
                                    principal={Principal.fromUint8Array(to.principal).toString()}
                                />
                            </Link>
                        </HStack>
                    </VStack>
                </GridItem>
            </GridSystem>
        );
    }
};

export default TransactionContent;
