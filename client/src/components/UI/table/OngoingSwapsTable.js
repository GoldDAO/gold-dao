import { Box, Table, TableBody, TableCell, TableHead, TableRow, Typography } from '@mui/material';
import React from 'react';
import RefreshButton from '../button/Refresh';
import { useNft } from '@/components/hooks/useNFTs';
import { useEffect } from 'react';
import { useAllCanisters } from '@/components/hooks/useAllCanisters';
import { useState } from 'react';
import { CustomCircularProgress } from '../styled/common';
import { theme } from '@/theme/theme';

const Row = ({ token_id, g, gldt, sale_id }) => {
    const [status, setStatus] = useState();
    const [statusLoading, setStatusLoading] = useState(true);
    if (!statusLoading)
        return (
            <StyledTableRow>
                <StyledTableCell>{token_id}</StyledTableCell>
                <StyledTableCell>{g}</StyledTableCell>
                <StyledTableCell>{g * 100}</StyledTableCell>
                <StyledTableCell>
                    <RefreshButton
                        setStatusLoading={setStatusLoading}
                        setStatus={setStatus}
                        token_id={token_id}
                        g={g}
                        gldt={gldt}
                        sale_id={sale_id}
                    />
                    {statusLoading ? <CustomCircularProgress /> : status}
                </StyledTableCell>
            </StyledTableRow>
        );
};

const OngoingSwapsTable = () => {
    const actors = useAllCanisters();
    const nfts = useNft(actors);
    const count = 0;
    return (
        <Box sx={{ gridColumn: '1/12' }}>
            {nfts.isLoading ? (
                <Box
                    sx={{
                        gridColumn: 'span 13',
                        height: '500px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        flexDirection: 'column',
                    }}
                >
                    <CustomCircularProgress />
                    <Typography
                        sx={{
                            fontStyle: 'italic',
                            color: theme.colors.darkgrey,
                            marginTop: '20px',
                        }}
                    >
                        Retrieving NFTs...
                    </Typography>
                </Box>
            ) : (
                <Table>
                    <StyledTableHead></StyledTableHead>
                    <TableBody>
                        {nfts.nfts.map((e, i) => {
                            if (e.sale_id) {
                                count + 1;
                                return (
                                    <Row
                                        token_id={e.name}
                                        g={e.weight}
                                        key={i}
                                        sale_id={e.sale_id}
                                    ></Row>
                                );
                            }
                        })}
                        {count === 0 ? (
                            <StyledTableRow>
                                <StyledTableCell>No token found</StyledTableCell>
                            </StyledTableRow>
                        ) : (
                            ''
                        )}
                    </TableBody>
                </Table>
            )}
        </Box>
    );
};

export default OngoingSwapsTable;

const TableContainer = styled(Box)`
    border-radius: 20px;
    grid-column: 1/13;
    border: 1px solid ${theme.colors.gold};
`;

const StyledTableRow = styled(TableRow)`
    display: table;
    width: 100%;
`;
const StyledCheckbox = styled(Checkbox)``;

const StyledTableHead = styled(TableHead)`
    font-weight: 400;
    display: table;
    width: 100%;
    border-radius: 20px 20px 0 0;
    background-color: ${theme.colors.grey};
`;
const StyledTableCell = styled(TableCell)`
    font-weight: inherit;
`;

const StyledTable = styled(Table)`
    display: block;
    border-radius: 20px;
`;
const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;

const CustomTableBody = styled(TableBody)`
    height: 400px;
    overflow: scroll;
    display: block;
`;
