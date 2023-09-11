import { Box, CircularProgress, Table, TableCell, TableHead, TableRow } from '@mui/material';
import React from 'react';
import RefreshButton from '../button/Refresh';
import { useNft, useOngoingSwaps } from '@/components/hooks/useNFTs';
import { useEffect } from 'react';
import { useAllCanisters } from '@/components/hooks/useAllCanisters';
import { useState } from 'react';

const Row = ({ token_id, g, gldt, sale_id }) => {
    const [status, setStatus] = useState();
    const [statusLoading, setStatusLoading] = useState(true);
    return (
        <TableRow>
            <TableCell>{token_id}</TableCell>
            <TableCell>{g}</TableCell>
            <TableCell>{g * 100}</TableCell>
            <TableCell>
                <RefreshButton
                    setStatusLoading={setStatusLoading}
                    setStatus={setStatus}
                    token_id={token_id}
                    g={g}
                    gldt={gldt}
                    sale_id={sale_id}
                />
                {statusLoading ? <CircularProgress /> : status}
            </TableCell>
        </TableRow>
    );
};

const OngoingSwapsTable = () => {
    const actors = useAllCanisters();
    const nfts = useNft(actors);

    return (
        <Table sx={{ gridColumn: '1/12' }}>
            On going swaps
            <TableHead></TableHead>
            {nfts.isLoading ? (
                <CircularProgress />
            ) : (
                nfts.nfts.map((e, i) => {
                    if (e.sale_id) {
                        return <Row token_id={e.name} g={e.weight} sale_id={e.sale_id}></Row>;
                    }
                })
            )}
        </Table>
    );
};

export default OngoingSwapsTable;
