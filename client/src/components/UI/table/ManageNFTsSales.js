import { useCanister } from '@connect2ic/react';
import { Box, Table, TableBody, TableCell, TableRow, Typography } from '@mui/material';
import React, { useEffect } from 'react';
import styled from 'styled-components';
import { getAllNftsAtom } from '../../../states/nfts';
import { onSaleNftAtom } from '../../../states/onSalesNfts';
import { useAtom } from 'jotai';
import medium from './../../../../public/images/gold/100g.png'
import Image from 'next/image';

const ManageNFTsSales = () => {
    const [onSale,] = useAtom(onSaleNftAtom)
    console.log('onSale', onSale)
    return (
        <StyledTable size='small'>
            <TableBody>
                {onSale.map((e, i) => (
                    <Row row={e} key={i} />
                ))}
            </TableBody>
        </StyledTable>
    );
};
export default ManageNFTsSales;


// const CancelsaleButton = ({ token_id }) => {
//     const [actor10g] = useCanister('NFT_10G_CANISTER');
//     const handleDeleteSale = async () => {
//         const req = await actor10g.sale_batch_nft_origyn([{ end_sale: token_id }])
//         console.log('req', req)
//     }
//     return (
//         <button onClick={() => { handleDeleteSale() }}>
//             Cancel Sale
//         </button >
//     )
// }

const Row = ({ row, }) => {
    return (
        <StyledTableRow key={row.name}>
            <StyledTableCell key="weight" padding="checkbox">
                {row.weight} {row.unit}
            </StyledTableCell>
            <StyledTableCell key="item">
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Image src={medium} alt={"NFT IMAGE"} />
                    <ItemName>{row.name}</ItemName>
                </Box>
            </StyledTableCell>
            <StyledTableCell key="item">
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    {/* <CancelsaleButton token_id={row.name} /> */}
                </Box>
            </StyledTableCell>
        </StyledTableRow>
    )
}


const StyledTableRow = styled(TableRow)`
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
