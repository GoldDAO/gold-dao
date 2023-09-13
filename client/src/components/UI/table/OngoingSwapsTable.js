import {
    Box,
    Checkbox,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableRow,
    Typography,
} from '@mui/material';
import RefreshButton from '../button/Refresh';
import { useEffect } from 'react';
import { useState } from 'react';
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
import styled from 'styled-components';
import { gldNftCanisters } from '@/services/agents';

const Row = ({ token_id, g, gldt, sale_id }) => {
    const [status, setStatus] = useState('awaiting status...');
    const [statusLoading, setStatusLoading] = useState(true);

    return (
        <StyledTableRow
            sx={{
                display: 'flex',
                alignItems: 'center',
            }}
        >
            <StyledTableCell key="token_id" sx={{ borderBottom: 0 }}>
                {token_id}
            </StyledTableCell>
            <StyledTableCell key="weight" sx={{ borderBottom: 0 }}>
                {g}
            </StyledTableCell>
            <StyledTableCell key="GLDT" sx={{ borderBottom: 0 }}>
                {g * 100}
            </StyledTableCell>
            <StyledTableCell
                key="status"
                sx={{ height: '100px', display: 'flex', alignItems: 'center', borderBottom: 0 }}
            >
                <RefreshButton
                    isLoading={statusLoading}
                    status={status}
                    setStatusLoading={setStatusLoading}
                    setStatus={setStatus}
                    token_id={token_id}
                    g={g}
                    gldt={gldt}
                    sale_id={sale_id}
                />
            </StyledTableCell>
        </StyledTableRow>
    );
};

const OngoingSwapsTable = ({ res }) => {
    const canisters = [1, 10, 100, 1000];

    return (
        <TableContainer>
            <StyledTableHead>
                <StyledTableRow>
                    <StyledTableCell key="token_id">Token ID</StyledTableCell>
                    <StyledTableCell key="weight">weight</StyledTableCell>
                    <StyledTableCell key="GLDT">GLDT</StyledTableCell>
                    <StyledTableCell key="status">Status</StyledTableCell>
                </StyledTableRow>
            </StyledTableHead>
            <StyledTable sx={{ gridColumn: '1/12' }}>
                {res?.map((e, g) => {
                    return e?.map((el, i) => {
                        if (el.ok) {
                            return (
                                <Row
                                    token_id={el.ok.token_id}
                                    g={canisters[g]}
                                    key={i}
                                    sale_id={el.ok.txn_type.sale_opened.sale_id}
                                />
                            );
                        }
                    });
                })}
            </StyledTable>
        </TableContainer>
    );
};

export default OngoingSwapsTable;

const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;
