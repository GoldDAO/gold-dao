import {
    Box,
    Button,
    Card,
    Flex,
    FormLabel,
    HStack,
    Heading,
    Input,
    InputGroup,
    InputRightElement,
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
import { useGldtLedgerTransactions } from '@utils/hooks/useGLDT';
import Timestamp from '@ui/tooltip/timeStamp';
import { Principal } from '@dfinity/principal';
import { ArrowBackIcon, ArrowForwardIcon, Search2Icon } from '@chakra-ui/icons';
import Link from 'next/link';
import PrincipalFormat from '../Principal';
import TokenSign from '@ui/gldt/TokenSign';
import Grid from '../Grid';

const Explorer = () => {
    const [search, setSearch] = useState();
    const [searchResult, setSearchResult] = useState();
    const [loading, setLoading] = useState(false);
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const { transactions, max } = useGldtLedgerTransactions(rowsPerPage, currentPage);
    return (
        <VStack
            gridColumn={['1/13', '1/13', '3/11', '3/11']}
            my="100px"
            alignItems={'flex-start'}
            spacing="20px"
        >
            <Heading
                fontWeight={300}
                as="h2"
                fontSize={'16px'}
                w={'100%'}
                borderBottom="1px"
                borderBottomColor={'secondaryText'}
            >
                Transactions
            </Heading>
            {/* <Grid>
                <InputGroup
                    display={'flex'}
                    alignItems={'center'}
                    gridColumn={['1/12', '1/12', '2/11', '3/10']}
                >
                    <Input size={'lg'} borderRadius={'50px'} />
                    <InputRightElement pointerEvents="none">
                        <Search2Icon color="gray.300" />
                    </InputRightElement>
                </InputGroup>
            </Grid> */}

            <TableContainer width={'100%'} m="0 auto" p="20px" bg="bg" borderRadius={'md'}>
                <Table bg="white" borderRadius={'sm'}>
                    <Thead>
                        <Tr
                            fontWeight={600}
                            color={'secondaryText'}
                            textTransform={'uppercase'}
                            fontSize={'12px'}
                        >
                            <Td>Type</Td>
                            <Td>Date/hour</Td>
                            <Td>From</Td>
                            <Td>To</Td>
                            <Td>Amount</Td>
                        </Tr>
                    </Thead>
                    <Tbody fontSize={'14px'}>
                        {transactions?.map((e, i) => {
                            return (
                                <Tr key={i}>
                                    <Td>{e.kind}</Td>
                                    <Td>
                                        <Timestamp timestamp={parseInt(e.timestamp)} />
                                    </Td>
                                    <Td>
                                        {e.kind === 'transfer' && (
                                            <Link
                                                href={`/accound-id/${Principal.fromUint8Array(
                                                    e[e.kind][0].from.owner._arr,
                                                ).toString()}`}
                                            >
                                                <PrincipalFormat
                                                    principal={Principal.fromUint8Array(
                                                        e[e.kind][0].from.owner._arr,
                                                    ).toString()}
                                                />
                                            </Link>
                                        )}
                                    </Td>
                                    <Td>
                                        <Linkaccound-id
                                            href={`/accound-id/${Principal.fromUint8Array(
                                                e[e.kind][0].to.owner._arr,
                                            ).toString()}`}
                                        >
                                            <PrincipalFormat
                                                principal={Principal.fromUint8Array(
                                                    e[e.kind][0].to.owner._arr,
                                                ).toString()}
                                            />
                                        </Link>
                                    </Td>
                                    <Td>
                                        <HStack>
                                            <Text>
                                                {(
                                                    parseInt(e[e.kind][0].amount) / 100000000
                                                ).toFixed(2)}
                                            </Text>{' '}
                                            <TokenSign />
                                        </HStack>
                                    </Td>
                                </Tr>
                            );
                        })}
                    </Tbody>
                </Table>
                <Pagination
                    total={parseInt(max?.log_length)}
                    currentHistoryPage={currentPage}
                    setCurrentHistoryPage={setCurrentPage}
                />
            </TableContainer>
        </VStack>
    );
};

export default Explorer;

const Pagination = ({ currentHistoryPage, setCurrentHistoryPage, total }) => {
    total = total ? total : 0;
    return (
        <VStack pt="20px">
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Text>Page {currentHistoryPage + 1}</Text>
                <Text>{total} entries</Text>
            </Flex>
            <Flex justifyContent={'space-between'} width={'100%'}>
                <Button
                    bg="white"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={currentHistoryPage < 1}
                    onClick={() => setCurrentHistoryPage((prev) => prev - 1)}
                >
                    <ArrowBackIcon />
                </Button>
                <Button
                    bg="white"
                    _hover={{
                        bg: 'border',
                    }}
                    isDisabled={total / (currentHistoryPage + 1) < 10}
                    onClick={() => setCurrentHistoryPage((prev) => prev + 1)}
                >
                    <ArrowForwardIcon />
                </Button>
            </Flex>
        </VStack>
    );
};
