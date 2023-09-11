import { Box, Table, TableCell, TableHead, TableRow } from '@mui/material';
import React from 'react';
import RefreshButton from '../button/Refresh';

const Row = ({ token_id, g, gldt, status }) => {
    return (
        <TableRow>
            <TableCell>{token_id}</TableCell>
            <TableCell>{g}</TableCell>
            {/* <TableCell>{gldt}</TableCell> */}
            <TableCell>
                <RefreshButton token_id={token_id} g={g} gldt={gldt} status={status} />
            </TableCell>
        </TableRow>
    );
};

const OngoingSwapsTable = ({ on_sale }) => {
    return (
        <Table sx={{ gridColumn: '1/12' }}>
            On going swaps
            <TableHead></TableHead>
            {on_sale.length > 0 &&
                on_sale.map((e, i) => <Row token_id={e.name} g={e.weight} status={e.status}></Row>)}
        </Table>
    );
};

export default OngoingSwapsTable;
