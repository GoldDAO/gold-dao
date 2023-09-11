import { Box, Table, TableCell, TableRow, Typography } from '@mui/material';
import React from 'react';

const SummaryTable = ({ g, nfts, minted, fees, received }) => {
    return (
        <Box>
            <Typography>Summary</Typography>
            <Box>
                <Table>
                    <TableRow>
                        <TableCell>Gold Amount</TableCell>
                        <TableCell>{g}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Nfts</TableCell>
                        <TableCell>
                            {nfts.map((e) => (
                                <Box>{e.name}</Box>
                            ))}
                        </TableCell>
                    </TableRow>
                    {/* <TableRow>
                        <TableCell>Minted tokens</TableCell>
                        <TableCell>{minted}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Minting fees</TableCell>
                        <TableCell>{fees}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Received tokens</TableCell>
                        <TableCell>{received}</TableCell>
                    </TableRow> */}
                </Table>
            </Box>
        </Box>
    );
};

export default SummaryTable;
