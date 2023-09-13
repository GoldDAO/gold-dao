import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import {
    Checkbox,
    CircularProgress,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TablePagination,
    TableRow,
    Typography,
} from '@mui/material';
import { useGldtLedgerTransactions } from '@/components/hooks/useGLDT';
import { Principal } from '@dfinity/principal';
import { useState } from 'react';
import { useEffect } from 'react';
import Timestamp from '../tooltip/timestamp';
import {
    CustomCircularProgress,
    CustomTableBody,
    StyledTable,
    StyledTableCell,
    StyledTableHead,
    StyledTableRow,
    TableContainer,
} from '../styled/common';
import { theme } from '@/theme/theme';

const TransactionsTable = () => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(10);
    const [loading, setLoading] = useState(true);
    const { transactions, max } = useGldtLedgerTransactions(rowsPerPage, currentPage);

    useEffect(() => {
        transactions ? setLoading(false) : setLoading(true);
        // console.log('transactions', transactions);
    }, [transactions]);

    const handleChangePage = (e, newPage) => {
        setCurrentPage(newPage);
    };

    const handleChangeRowsPerPage = (e) => {
        setRowsPerPage(parseInt(e.target.value));
        setCurrentPage(0);
    };

    // const tableHead = [Object.keys(transactions[0] ? transactions[0] : [])]
    const tableHead = ['Kind', 'Timestamp', 'Amount', 'From', 'To'];

    if (!loading) {
        return (
            <TableContainer sx={{ gridColumn: 'span 12' }}>
                <StyledTable>
                    <StyledTableHead>
                        <StyledTableRow>
                            {tableHead.map((e, i) => {
                                return <StyledTableCell key={e}>{e}</StyledTableCell>;
                            })}
                        </StyledTableRow>
                    </StyledTableHead>
                    <CustomTableBody height="600px">
                        {transactions?.map((e, i) => {
                            return <Row key={tableHead[i]} row={e} />;
                        })}
                    </CustomTableBody>
                </StyledTable>
                <TablePagination
                    rowsPerPageOptions={[5, 15, 25]}
                    component="div"
                    count={parseInt(max?.log_length)}
                    rowsPerPage={rowsPerPage}
                    page={currentPage}
                    onPageChange={handleChangePage}
                    onRowsPerPageChange={handleChangeRowsPerPage}
                />
            </TableContainer>
        );
    } else
        return (
            <Box
                sx={{
                    gridColumn: 'span 12',
                    height: '500px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'column',
                }}
            >
                <CustomCircularProgress />
                <Typography
                    sx={{ marginTop: '20px', fontStyle: 'italic', color: theme.colors.darkgrey }}
                >
                    Loading transactions...
                </Typography>
            </Box>
        );
};

export default TransactionsTable;

const Row = ({ row }) => {
    let formatedRow;
    switch (row.kind) {
        case 'mint':
            formatedRow = {
                Kind: row.kind,
                Timestamp: parseInt(row.timestamp),
                Amount: (parseInt(row.mint[0].amount) / 10 ** 8).toFixed(8),
                From: 'Minting Account',
                To: (
                    <>
                        <p>
                            Principal:{' '}
                            {Principal.fromUint8Array(row.mint[0].to.owner._arr).toString()}
                        </p>
                    </>
                ),
            };
            break;
        case 'transfer':
            formatedRow = {
                Kind: row.kind,
                Timestamp: parseInt(row.timestamp),
                Amount: (parseInt(row.transfer[0].amount) / 10 ** 8).toFixed(8),
                From: (
                    <>
                        <p>
                            {Principal.fromUint8Array(row.transfer[0].from.owner._arr).toString()}
                        </p>
                    </>
                ),
                To: (
                    <>
                        <p>{Principal.fromUint8Array(row.transfer[0].to.owner._arr).toString()}</p>
                    </>
                ),
            };
            break;
    }

    return (
        <StyledTableRow>
            {Object.keys(formatedRow).map((e, i) => {
                return (
                    <StyledTableCell key={i}>
                        {e === 'Timestamp' ? (
                            <Timestamp timestamp={formatedRow[e]} />
                        ) : (
                            formatedRow[e]
                        )}
                    </StyledTableCell>
                );
            })}
        </StyledTableRow>
    );
};

const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;
