import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import medium from './../../../../public/images/gold/100g.png';
import {
    Checkbox,
    CircularProgress,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableRow,
    Typography,
} from '@mui/material';
import Image from 'next/image';
import { useNft } from '@/components/hooks/useNFTs';
import { useAllCanisters } from '@/components/hooks/useAllCanisters';
import NftControls from './NftStatus';
import {
    addAllItemsAtom,
    addCartItemAtom,
    getCartAtom,
    removeAllItemsInCartAtom,
    removeCartItemByIdAtom,
} from '@/states/cart';

const MyNfts = ({ hasControls, selectable }) => {
    const actors = useAllCanisters();
    const nfts = useNft(actors);

    const [cart] = useAtom(getCartAtom);
    const [, removeAllItems] = useAtom(removeAllItemsInCartAtom);
    const [, addAllNFTsInCart] = useAtom(addAllItemsAtom);
    const [isAllSelected] = useState(false);

    const tableHead = [
        {
            key: 'weight',
            label: 'Weight',
        },
        {
            key: 'name',
            label: 'name',
        },
    ];
    if (!nfts.isLoading) {
        return (
            <Box sx={{ width: '100%' }}>
                <StyledTable>
                    <StyledTableHead>
                        <StyledTableRow>
                            {selectable && (
                                <StyledTableCell padding="checkbox">
                                    <StyledCheckbox
                                        onChange={(e) => {
                                            e.target.checked
                                                ? addAllNFTsInCart(nfts.nfts)
                                                : removeAllItems();
                                        }}
                                        inputProps={{ 'aria-label': 'select all NFTs' }}
                                    />
                                </StyledTableCell>
                            )}
                            {tableHead.map((e, i) => (
                                <StyledTableCell key={e.key}>{e.label}</StyledTableCell>
                            ))}
                            {hasControls && (
                                <StyledTableCell key="controls">Status</StyledTableCell>
                            )}
                        </StyledTableRow>
                    </StyledTableHead>
                    <TableBody>
                        {nfts?.nfts?.map((nft, i) => {
                            return (
                                <Row
                                    cart={cart}
                                    isAllSelected={isAllSelected}
                                    hasControls={hasControls}
                                    selectable={selectable}
                                    key={i}
                                    row={nft}
                                />
                            );
                        })}
                    </TableBody>
                </StyledTable>
            </Box>
        );
    } else if (nfts.isLoading) {
        return (
            <Box
                sx={{
                    width: '100%',
                    height: '500px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'column',
                }}
            >
                <CircularProgress />
                <Typography sx={{ marginTop: '20px' }}>Loading Nfts...</Typography>
            </Box>
        );
    }
};

export default MyNfts;

const Row = ({ row, hasControls, selectable, isAllSelected, cart }) => {
    const [, setCartItem] = useAtom(addCartItemAtom);
    const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom);
    const [isInCart, setIsInCart] = useState(false);

    useEffect(() => {
        const a = cart.find((e) => e?.name === row.name);
        setIsInCart(a ? true : false);
    }, [cart]);

    return (
        <StyledTableRow>
            {selectable && (
                <StyledTableCell padding="checkbox">
                    <StyledCheckbox
                        onChange={(e) => {
                            e.target.checked ? setCartItem(row) : removeItemFromCart(row.name);
                        }}
                        inputProps={{ 'aria-label': `select NFTs with name ${row.name}` }}
                        checked={isInCart}
                    />
                </StyledTableCell>
            )}
            <StyledTableCell key="weight" padding="checkbox">
                {row.weight} {row.unit}
            </StyledTableCell>
            <StyledTableCell>
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <Image src={medium} alt={'NFT IMAGE'} />
                    <ItemName>{row.name}</ItemName>
                </Box>
            </StyledTableCell>
            <StyledTableCell>
                {hasControls && (
                    <NftControls token_id={row.name} weight={row.weight} onSale={row.status} />
                )}
            </StyledTableCell>
        </StyledTableRow>
    );
};

const MarketCapContainer = styled(Box)`
    font-size: 1em;
    background-color: #f7f7f7;
    width: fit-content;
    padding: 20px;
    border-radius: 10px;
    display: flex;
    @media (max-width: 940px) {
        font-size: 0.8em;
    }
    @media (max-width: 540px) {
        width: 100%;
    }
`;
const StyledTableRow = styled(TableRow)``;
const StyledCheckbox = styled(Checkbox)``;

const StyledTableHead = styled(TableHead)`
    font-weight: 600;
`;
const StyledTableCell = styled(TableCell)`
    font-weight: inherit;
`;

const StyledTable = styled(Table)``;
const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;
