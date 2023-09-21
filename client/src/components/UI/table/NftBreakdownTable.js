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
import { Principal } from '@dfinity/principal';
import { useState } from 'react';
import { useEffect } from 'react';
import Timestamp from '../tooltip/timestamp';
import { useCanister } from '@connect2ic/react';
import { gldtCoreCanister } from '@/services/agents';
import { useRecords } from '@/components/hooks/useRecords';
import { CustomCircularProgress } from '../styled/common';

const NftBreakdownTable = () => {
    const [currentPage, setCurrentPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(5);
    const [loading, setLoading] = useState(false);

    const records = useRecords(rowsPerPage, currentPage);

    const handleChangePage = (e, newPage) => {
        setCurrentPage(newPage);
    };

    const handleChangeRowsPerPage = (e) => {
        setRowsPerPage(parseInt(e.target.value));
        setCurrentPage(0);
    };

    const tableHead = [
        'Nft ID',
        'Record Type',
        'grams',
        'Minting TimeStamp',
        'GLDT Minted',
        'Receiving Account',
        'GLDNFT Canister',
        'Block',
    ];

    if (!loading) {
        return (
            <Box sx={{ gridColumn: 'span 12' }}>
                <StyledTable>
                    <StyledTableHead>
                        <StyledTableRow>
                            {tableHead.map((e, i) => {
                                return <StyledTableCell key={i}>{e}</StyledTableCell>;
                            })}
                        </StyledTableRow>
                    </StyledTableHead>
                    <TableBody>
                        {records.records.map((e, i) => {
                            return <Row key={i} row={e} />;
                        })}
                    </TableBody>
                </StyledTable>
                <TablePagination
                    rowsPerPageOptions={[5, 15, 25]}
                    component="div"
                    rowsPerPage={rowsPerPage}
                    count={parseInt(records.total)}
                    page={currentPage}
                    onPageChange={handleChangePage}
                    onRowsPerPageChange={handleChangeRowsPerPage}
                />
            </Box>
        );
    } else
        return (
            <Box
                sx={{
                    width: '100%',
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
                    Loading Nfts...
                </Typography>
            </Box>
        );
};

export default NftBreakdownTable;

const Row = ({ row }) => {
    const tableHead = [
        'Nft ID',
        'Record Type',
        'grams',
        'Minting TimeStamp',
        'GLDT Minted',
        'Receiving Account',
        'GLDNFT Canister',
        'Block',
    ];
    const formatedRow = {
        nft_id: row.nft_id,
        record_type: row.record_type,
        grams: parseInt(row.grams),
        gldt_minting_timestamp_seconds: parseInt(row.gldt_minting_timestamp_seconds) * 1000000000,
        gldt_minted: row.gldt_minted,
        receiving_account: row.receiving_account,
        gld_nft_canister_id: row.gld_nft_canister_id,
        block_height: row.block_height,
    };
    return (
        <StyledTableRow>
            {Object.keys(formatedRow).map((e, i) => {
                switch (e) {
                    case 'receiving_account':
                        return (
                            <StyledTableCell key={e}>
                                <p>
                                    Principal:
                                    {Principal.fromUint8Array(formatedRow[e].owner._arr).toString()}
                                </p>
                                <p>SubAccount: [{formatedRow[e].subaccount[0]?.join(', ')}]</p>
                            </StyledTableCell>
                        );
                    case 'gldt_minting_timestamp_seconds':
                        return (
                            <StyledTableCell key={e}>
                                <Timestamp timestamp={formatedRow[e]} />
                            </StyledTableCell>
                        );
                    case 'gld_nft_canister_id':
                        return (
                            <StyledTableCell key={e}>
                                <p>{Principal.fromUint8Array(formatedRow[e]._arr).toString()}</p>
                            </StyledTableCell>
                        );
                    default:
                        return (
                            <StyledTableCell key={e}>{formatedRow[e].toString()}</StyledTableCell>
                        );
                }
            })}
        </StyledTableRow>
    );
};

const StyledTableRow = styled(TableRow)``;
const StyledCheckbox = styled(Checkbox)``;

const StyledTableHead = styled(TableHead)`
    font-weight: 600;
`;
const StyledTableCell = styled(TableCell)`
    font-weight: inherit;
`;

const StyledTable = styled(Table)``;
const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;
