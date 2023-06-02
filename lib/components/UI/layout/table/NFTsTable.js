import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import { TableRow, Table, TableCell, Checkbox, TableHead, Typography, TableBody } from '@mui/material';
import Image from 'next/image';
import { Box } from '@mui/system';
import { useAtom } from 'jotai'
import { addAllItemsAtom, addCartItemAtom, getCartAtom, removeAllItemsInCartAtom, removeCartItemByIdAtom } from '../../../../sequence/atoms/cart';
import { nftsAtom } from '../../../../sequence/atoms/nfts';

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

const NFTTable = () => {
    const [nfts] = useAtom(nftsAtom)
    const [cart,] = useAtom(getCartAtom)
    const [, removeAllItems] = useAtom(removeAllItemsInCartAtom)
    const [, addAllNFTsInCart] = useAtom(addAllItemsAtom)
    const [isAllSelected, setIsAllSelected] = useState(false)

    useEffect(() => {
        if (cart.length === nfts.length) {
            setIsAllSelected(true)
        } else { setIsAllSelected(false) }
    }, [cart])

    return (
        <StyledTable size='small'>
            <StyledTableHead>
                <StyledTableRow>
                    <StyledTableCell padding="checkbox">
                        <StyledCheckbox
                            onChange={(e) => {
                                e.target.checked ? addAllNFTsInCart() :
                                    removeAllItems()
                            }}
                            checked={isAllSelected}
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
    const [isInCart, setIsInCart] = useState(false)
    const [cart,] = useAtom(getCartAtom)
    const [, setCartItem] = useAtom(addCartItemAtom)
    const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom)

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
                            setCartItem(row) :
                            removeItemFromCart(row.id)
                    }}
                    inputProps={{ 'aria-label': `select NFTs with id ${row.id}`, }}
                    checked={isInCart}
                />
            </StyledTableCell>
            <StyledTableCell key="weight" padding="checkbox">
                {row.weight} {row.unit}
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