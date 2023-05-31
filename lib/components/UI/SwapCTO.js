import { Box } from '@mui/system';
import React, { useState } from 'react';
import styled from 'styled-components'
import { Inter } from 'next/font/google';
import { useAtom } from 'jotai'
import { cartAtom } from '../../sequence/cart';
import MainButton from './layout/button/MainButton';
import CustomDialog from './layout/Dialog';

const inter = Inter({ subsets: ['latin'] });

const SwapCTO = ({ data, setOpen, open }) => {
    const [cart, setCart] = useAtom(cartAtom)
    const [validationDialog, setValidationDialog] = useState(false)

    const getTotalWeight = () => {
        let weight = 0;
        if (cart.length)
            cart.map((e, i) => {
                weight += e.weight
            })
        return weight
    }

    return (
        <SwapCard>
            <Title>{data.title}</Title>
            <Box sx={{ display: 'flex', width: '100%', justifyContent: 'space-between', alignItems: "center", margin: '20px 0' }}>
                <GLDNFT> {getTotalWeight()}
                    <button
                        onClick={() => setOpen(true)}
                        className={inter.className}
                        style={{
                            color: '#333',
                            display: 'flex',
                            cursor: 'pointer',
                            justifyContent: 'center',
                            alignItems: 'center',
                            flexDirection: 'column',
                            padding: '20px',
                            borderRadius: "8px",
                            margin: '0 20px'
                        }}>
                        {data.buttonLabel}
                        <span style={{
                            fontSize: '48px',
                            fontWeight: 600
                        }}>{data.inputCurrency}
                        </span>
                    </button>
                </GLDNFT>
                <Box sx={{
                    padding: '50px',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    color: '#231F20',
                    backgroundColor: '#DEDEDE',
                    height: '100px',
                    width: '100px',
                    fontSize: "1.5em",
                    borderRadius: "10px",
                    margin: '0 60px'
                }}>
                    →
                </Box>
                <GLDT>
                    <Box sx={{
                        padding: "40px",
                        height: '100%'
                    }}>0 <span style={{
                        fontWeight: 600
                    }}>{data.outputCurrency}</span> </Box>
                </GLDT>
            </Box>
            <Box>{data.value}</Box>
            {cart.length > 0 &&
                <Box sx={{
                    width: '100%',
                    display: 'flex',
                    justifyContent: 'flex-end',
                    padding: '20px 0'
                }}>
                    <MainButton
                        label="Validate the transaction"
                        isInactive={!cart.length}
                        action={() => setValidationDialog(true)} />
                </Box>}
            <CustomDialog title="You are in the process to exchange your " open={validationDialog} setOpen={setValidationDialog} />
        </SwapCard >
    );
};



export default SwapCTO;

const SwapCard = styled(Box)`
    padding: 94px 76px 20px 76px;
    margin: 100px 0;
    background-color: #f2f2f2;
    border-radius: 30px; 
    display: flex;
    align-items: center;
    flex-direction: column;
    justify-content: center;
    box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
`

const GLDNFT = styled(Box)`
   background-color: #fff;
   box-shadow: inset 0px 4px 4px rgba(0, 0, 0, 0.25);
   width: 100%; 
   border-radius: 8px;
   display: flex;
   font-size: 1.9em;
   font-weight: 300;
   justify-content: flex-end;
   align-items: center;
   padding: 20px;
   border-radius: 8px;
`

const GLDT = styled(Box)`
    background-color: #fff;
    box-shadow: inset 0px 4px 4px rgba(0, 0, 0, 0.25);
    width: 100%; 
    font-weight: 300;
    border-radius: 8px;
    font-size: 1.9em;
    display: flex;
    justify-content: flex-end;
    height: 100%;    
    align-items: center;
    padding: 20px;
    border-radius: 8px;
`

const Title = styled(Box)`
    font-size: 3.2em;
    width: 100%;
    color: #333;
    font-weight: 600;
`