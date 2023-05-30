import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import { TableRow, Table, TableCell, Checkbox, TableHead, Typography, TableBody } from '@mui/material';
import Image from 'next/image';
import small from './../../../../../public/images/gold/10g.png'
import medium from './../../../../../public/images/gold/100g.png'
import big from './../../../../../public/images/gold/1kg.png'
import { Box } from '@mui/system';
import { useAtom } from 'jotai'
import { cartAtom } from '../../../../sequence/cart';

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
    { name: 'asdf', image: small, id: 'asdf-1', weight: '10g' },
    { name: 'asdf', image: small, id: 'asdf-2', weight: '10g' },
    { name: 'asdf', image: small, id: 'asdf-3', weight: '10g' },
    { name: 'asdf', image: small, id: 'asdf-4', weight: '10g' },
    { name: 'asdf', image: small, id: 'asdf-5', weight: '10g' },
]

const addItem = (item) => {

}

const removeItemById = (id, cart) => {
    cart.map((e, i) => {
        if (e.id === id) {
            delete cart[i]
        }
    })
    const filteredCart = cart.filter((e) => {
        return typeof e === 'object';
    });
    return filteredCart
}

const addAllItems = (items) => {

}

const removeAllItems = () => {

}


const NFTTable = () => {
    const [cart, setCart] = useAtom(cartAtom)
    console.log('cart', cart)

    return (
        <StyledTable size='small'>
            <StyledTableHead>
                <StyledTableRow>
                    <StyledTableCell padding="checkbox">
                        <StyledCheckbox
                            onChange={(e) => {
                                e.target.checked ? setCart(nfts) :
                                    setCart([])
                            }}
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
            <TableBody>
                {nfts.map((e, i) => (
                    <Row row={e} key={i} />
                ))}
            </TableBody>
        </StyledTable>
    );
};

export default NFTTable;

const Row = ({ row, }) => {
    const [cart, setCart] = useAtom(cartAtom)
    const [isInCart, setIsInCart] = useState(false)

    useEffect(() => {
        const a = cart.find(e => e?.id === row.id)
        setIsInCart(a ? true : false)
    }, [cart, row]);

    return (
        <StyledTableRow key={row.id}>
            <StyledTableCell padding="checkbox">
                <StyledCheckbox
                    onChange={(e) => {
                        e.target.checked ?
                            setCart([...cart, row]) :
                            setCart(removeItemById(row.id, cart))
                    }}
                    inputProps={{ 'aria-label': `select NFTs with id ${row.id}`, }}
                    checked={isInCart}
                />
            </StyledTableCell>
            <StyledTableCell key="weight" padding="checkbox">
                {row.weight}
            </StyledTableCell>
            <StyledTableCell key="item">
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Image src={row.image} alt={row.image.alt} />
                    <ItemName>{row.name}</ItemName>
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