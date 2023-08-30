import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import { Checkbox, CircularProgress, Table, TableBody, TableCell, TableHead, TablePagination, TableRow, Typography } from '@mui/material';
import { useGldtLedgerTransactions } from '@/components/hooks/useGLDT';
import { Principal } from '@dfinity/principal';
import { useState } from 'react';
import { useEffect } from 'react';

const TransactionsTable = () => {
    const [currentPage, setCurrentPage] = useState(0)

    const [rowsPerPage, setRowsPerPage] = useState(5)

    const { transactions, max } = useGldtLedgerTransactions(rowsPerPage, currentPage)

    useEffect(() => {
        console.log('currentPage', currentPage)
    }, [currentPage])

    useEffect(() => {
        console.log('rowsPerPage', rowsPerPage)
    }, [rowsPerPage])
    const handleChangePage = (e, newPage) => {
        setCurrentPage(newPage);
    };

    const handleChangeRowsPerPage = (e) => {
        setRowsPerPage(parseInt(e.target.value));
        setCurrentPage(0);
    };

    // const tableHead = [Object.keys(transactions[0] ? transactions[0] : [])]
    const tableHead = [
        'Kind',
        'Timestamp',
        'Amount',
        'To'
    ]
    return (
        <Box sx={{ width: '100%' }}>
            <StyledTable>
                <StyledTableHead>
                    <StyledTableRow>
                        {tableHead.map((e, i) => {
                            return (
                                <StyledTableCell key={i} >
                                    {e}
                                </StyledTableCell>
                            )
                        })}
                    </StyledTableRow>
                </StyledTableHead>
                <TableBody>
                    {transactions?.map((e, i) => {
                        return (
                            <Row
                                key={i}
                                row={e}
                            />
                        );
                    })}
                </TableBody>
            </StyledTable >
            <TablePagination
                rowsPerPageOptions={[5, 15, 25]}
                component="div"
                count={parseInt(max?.log_length)}
                rowsPerPage={rowsPerPage}
                page={currentPage}
                onPageChange={handleChangePage}
                onRowsPerPageChange={handleChangeRowsPerPage}
            />
        </Box>
    )
}

export default TransactionsTable;


const Row = ({ row }) => {
    const formatedRow = {
        Kind: row.kind,
        Timestamp: parseInt(row.timestamp),
        Amount: parseInt(row.mint[0].amount),
        To: Principal.fromUint8Array(row.mint[0].to.owner._arr).toString()
    }
    return (
        <StyledTableRow>
            {Object.keys(formatedRow).map((e, i) => {
                return (
                    <StyledTableCell key={i}>
                        {formatedRow[e]}
                    </StyledTableCell>
                )
            })}
        </StyledTableRow>
    )
}


const StyledTableRow = styled(TableRow)`
  `
const StyledCheckbox = styled(Checkbox)`
  `

const StyledTableHead = styled(TableHead)`
      font-weight: 600;
  `
const StyledTableCell = styled(TableCell)`
      font-weight: inherit;
  `

const StyledTable = styled(Table)`
  
  `
const ItemName = styled(Typography)`
      height: 100%;
      align-items: center;
      display: inline-flex;
      padding-left: 16px;
  
  `


