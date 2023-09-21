import { theme } from '@/theme/theme';
import { Box, Table, TableCell, TableRow, Typography } from '@mui/material';
import React from 'react';
import styled from 'styled-components';

const SummaryTable = ({ g, nfts }) => {
    const minted = g * 100;
    const fees = g * 100 * 0.02;
    const gldtTotal = minted - fees;

    return (
        <SummaryTableContainer>
            <Title>Summary</Title>
            <Box>
                <Table>
                    <SummaryTableRow>
                        <SummaryTableCell>Gold Amount</SummaryTableCell>
                        <SummaryTableCell>{g}g</SummaryTableCell>
                    </SummaryTableRow>
                    <SummaryTableRow>
                        <SummaryTableCell>Nfts</SummaryTableCell>
                        <SummaryTableCell style={{ width: 'fit-content' }}>
                            {nfts.map((e, i) => (
                                <>{(i > 0 ? ', ' : '') + e.name}</>
                            ))}
                        </SummaryTableCell>
                    </SummaryTableRow>
                    <SummaryTableRow>
                        <SummaryTableCell>Minted tokens</SummaryTableCell>
                        <SummaryTableCell>{minted} GLDT</SummaryTableCell>
                    </SummaryTableRow>
                    <SummaryTableRow>
                        <SummaryTableCell>Minting fees</SummaryTableCell>
                        <SummaryTableCell>{fees} GLDT</SummaryTableCell>
                    </SummaryTableRow>
                    <SummaryTableRow>
                        <SummaryTableCell>Received tokens</SummaryTableCell>
                        <SummaryTableCell>{gldtTotal} GLDT</SummaryTableCell>
                    </SummaryTableRow>
                </Table>
            </Box>
        </SummaryTableContainer>
    );
};

export default SummaryTable;

const SummaryTableContainer = styled(Box)`
    color: ${theme.colors.darkgrey};
`;

const SummaryTableRow = styled(TableRow)``;

const SummaryTableCell = styled(TableCell)`
    border: none;
    padding: 0 0 10px 0;
    width: 175px;
    display: inline-block;
`;

const Title = styled(Typography)`
    border-bottom: 1px solid ${theme.colors.darkgrey};
    margin-bottom: 20px;
`;
