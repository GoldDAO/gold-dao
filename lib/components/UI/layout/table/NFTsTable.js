import React, { useEffect } from 'react';
import styled from 'styled-components';
import { TableRow, Table, TableCell, Checkbox, TableHead, Typography } from '@mui/material';
import Image from 'next/image';
import small from './../../../../../public/images/gold/10g.png'
import medium from './../../../../../public/images/gold/100g.png'
import big from './../../../../../public/images/gold/1kg.png'
import { Box } from '@mui/system';



const tableHead = [
    {
        key: 'weight',
        label: 'Weight'
    },
    {
        key: 'item',
        label: 'Item'
    },
]

const nfts = [
    {
        item: { name: 'asdf', image: small, id: 'asdf-1', weight: '10g' },
    },
    {
        item: { name: 'asdf', image: small, id: 'asdf-2', weight: '10g' },
    },
    {
        item: { name: 'asdf', image: small, id: 'asdf-3', weight: '10g' },
    },
    {
        item: { name: 'asdf', image: small, id: 'asdf-4', weight: '10g' },
    },
    {
        item: { name: 'asdf', image: small, id: 'asdf-5', weight: '10g' },
    },
]

const cart = []


const NFTTable = () => {



    return (
        <StyledTable size='small'>
            <StyledTableHead>
                <StyledTableRow>
                    <StyledTableCell padding="checkbox">
                        <StyledCheckbox
                            onChange={() => cart.push(...nfts)}
                            inputProps={{ 'aria-label': 'select all NFTs', }}
                        />
                    </StyledTableCell>
                    {tableHead.map((e, i) => (
                        <StyledTableCell key={e.key} >
                            {e.label}
                        </StyledTableCell>
                    ))}
                </StyledTableRow>
            </StyledTableHead>
            {nfts.map((e, i) => (
                <Row row={e} key={i} />
            ))}
        </StyledTable>
    );
};

export default NFTTable;

const Row = ({ row }) => {

    return (
        <StyledTableRow key={row.id}>
            <StyledTableCell padding="checkbox">
                <StyledCheckbox
                    onChange={() => { cart.push(row) }}
                    inputProps={{ 'aria-label': `select NFTs with id ${row.id}`, }}
                />
            </StyledTableCell>
            <StyledTableCell key="weight" padding="checkbox">
                {row.item.weight}
            </StyledTableCell>
            <StyledTableCell key="item">
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Image src={row.item.image} alt={row.item.image.alt} />
                    <ItemName>{row.item.name}</ItemName>
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

const StyledTableHead = styled(TableHead)`
    font-weight: 600;
`
const StyledCheckbox = styled(Checkbox)`
`
const StyledTable = styled(Table)`


`
const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;

`