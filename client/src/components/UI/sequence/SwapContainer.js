import { Box } from '@mui/system';
import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import { Inter } from 'next/font/google';
import { useAtom } from 'jotai';
import MainButton from '../button/Buttons';
import ConfirmDialog from './ConfirmDialog';
import { cartAtom, getTotalCartWeightAtom } from '@/states/cart';
import Dialog from '../Dialog';
import NFTsTable from '../table/NFTsTable';
import { userAtom } from '@/states/user';
import { useDialog } from '@connect2ic/react';
import { Button, Typography } from '@mui/material';
import { SectionTitle } from '../common';

const inter = Inter({ subsets: ['latin'] });

const SwapContainer = ({ data }) => {
  const [cart] = useAtom(cartAtom);
  const [user] = useAtom(userAtom);
  const [total] = useAtom(getTotalCartWeightAtom);
  const [validationDialog, setValidationDialog] = useState(false);
  const [openCTO, setOpenCTO] = useState(false);
  const { open } = useDialog();

  return (
    <>
      <SectionTitle>
        Start trading your <span style={{ color: '#d3b872' }}>GLDNFTs</span> now !
      </SectionTitle>
      <SwapCard>
        <Title>
          Swap your <Mark>GLDNFT</Mark> for <Mark>GLDT</Mark>
        </Title>
        <SwapModalContentContainer sx={{ display: 'flex', alignItems: 'center' }}>
          <GLDNFT>
            <ValueContainer>
              {' '}
              {total}
              <span style={{ fontSize: '24px' }}>g </span>{' '}
              <Mark
                style={{
                  fontWeight: 600,
                  display: 'block',
                }}
              >
                GLDNFT
              </Mark>{' '}
            </ValueContainer>
          </GLDNFT>
          <LeftArrow>→</LeftArrow>
          <GLDT>
            <ValueContainer>
              {total * 100}{' '}
              <Mark
                style={{
                  fontWeight: 600,
                  display: 'block',
                }}
              >
                {data.outputCurrency}
              </Mark>{' '}
            </ValueContainer>
          </GLDT>
        </SwapModalContentContainer>
        <ButtonContainer>
          <AddGLDNFT
            onClick={() => (user.principal ? setOpenCTO(true) : open())}
            className={inter.className}
            style={{
              color: '#333',
              display: 'flex',
              cursor: 'pointer',
              justifyContent: 'center',
              alignItems: 'center',
              flexDirection: 'column',
              padding: '20px 40px',
              borderRadius: '25px',
            }}
          >
            <span sx={{ fontSize: '12px' }}>{data.buttonLabel}</span>
            <span className="GLDNFT">{data.inputCurrency}</span>
          </AddGLDNFT>
        </ButtonContainer>
        <Box
          style={{
            backgroundColor: '#f7f7f7',
            borderRadius: '15px',
            padding: '15px',
            color: '#444444',
            fontSize: '16px',
            marginBottom: '20px',
          }}
        >
          {data.value}
        </Box>
        {cart.length > 0 && (
          <Box
            sx={{
              margin: '0 auto',
              paddingBottom: '40px',
            }}
          >
            <MainButton
              style={{ width: '100%' }}
              label="Validate the transaction"
              isInactive={!cart.length}
              action={() => setValidationDialog(true)}
            />
          </Box>
        )}

        {user.isConnected && (
          <Dialog
            title="Select your GLD NFT(s) you want to swap for GLDT"
            address={user.principal}
            open={openCTO}
            setOpen={setOpenCTO}
            isButton={true}
            content={<NFTsTable />}
          />
        )}
        <ConfirmDialog total={total} open={validationDialog} setOpen={setValidationDialog} />
      </SwapCard>
    </>
  );
};

export default SwapContainer;

const Mark = styled('span')`
  color: #d3b872;
  font-weight: 500;
`;

const SwapCard = styled(Box)`
  background-color: #fff;
  border-radius: 30px;
  display: flex;
  align-items: center;
  border: 1px solid #d3b872;
  flex-direction: column;
  justify-content: center;
  margin: 80px auto;
  max-width: 800px;
  box-shadow: 0px 0px 90px 10px rgba(0, 0, 0, 0.08);
  padding-bottom: 40px;
  @media (max-width: 1140px) {
    margin: 60px auto;
  }
  @media (max-width: 840px) {
    margin: 60px auto;
  }
  @media (max-width: 480px) {
    margin: 40px auto;
  }
`;

const GLDNFT = styled(Box)`
  background-color: #fff;
  border-radius: 8px;
  display: flex;
  font-size: 1.9em;
  font-weight: 300;
  justify-content: flex-end;
  align-items: center;
  border-radius: 8px;
  text-align: right;
  width: 25%;
  justify-content: center;
  @media (max-width: 940px) {
    width: 100%;
    text-align: center;
    font-size: 1.3em;
  }
`;

const GLDT = styled(Box)`
  background-color: #fff;
  font-weight: 300;
  border-radius: 8px;
  font-size: 1.9em;
  display: flex;
  justify-content: flex-start;
  height: 100%;
  align-items: center;
  justify-content: center;
  padding: 20px;
  border-radius: 8px;
  width: 25%;
  @media (max-width: 940px) {
    width: 100%;
    text-align: center;
    font-size: 1.3em;
  }
`;

const Title = styled(Box)`
  background-color: #f7f7f7;
  padding: 40px;
  border-bottom: 1px solid #d3b872;
  width: 100%;
  font-weight: 500;
  font-size: 1.35em;
  margin-bottom: 25px;
  color: #333;
  border-radius: 30px 30px 0 0;
  @media (max-width: 1140px) {
    font-size: 1.3em;
  }
  @media (max-width: 840px) {
    font-size: 1.2em;
    padding: 30px;
  }
  @media (max-width: 480px) {
    font-size: 1em;
    padding: 20px;
  }
`;

const AddGLDNFT = styled(Button)`
  &.MuiButton-root {
    background-color: #d3b872;
    position: relative;
    z-index: 2;
    margin: 0 auto;
    width: 100%;
    span {
      color: #fff;
      font-size: 0.8em;
    }
    .GLDNFT {
      font-size: 1.7em;
      line-height: 1em;
      font-weight: 500;
    }
  }
  &.MuiButton-root:hover {
    background-color: #b29b60;
  }
`;

const SwapModalContentContainer = styled(Box)`
  display: flex;
  align-items: center;
  width: 100%;
  justify-content: center;
  @media (max-width: 1140px) {
  }
  @media (max-width: 940px) {
    flex-direction: column;
  }
  @media (max-width: 480px) {
  }
`;

const RightArrow = styled('p')`
  position: relative;
  left: -10px;
  font-size: 1.3em;
  z-index: 1;
  font-weight: 300;
  font-family: 'Inter', sans-serif;
  @media (max-width: 940px) {
    display: none;
  }
`;

const LeftArrow = styled('p')`
  position: relative;
  right: -15px;
  font-family: 'Inter', sans-serif;
  font-size: 1.3em;
  z-index: 1;
  font-weight: 300;
  @media (max-width: 940px) {
    display: none;
  }
`;

const TopArrow = styled('p')`
  position: relative;
  bottom: -27px;
  font-family: 'Inter', sans-serif;
  display: none;
  font-size: 1.3em;
  z-index: 1;
  font-weight: 300;
  @media (max-width: 940px) {
    display: block;
  }
`;

const BottomArrow = styled('p')`
  position: relative;
  top: -18px;
  font-size: 1.3em;
  font-family: 'Inter', sans-serif;
  display: none;
  z-index: 1;
  font-weight: 300;
  @media (max-width: 940px) {
    display: block;
  }
`;

const ButtonContainer = styled(Box)`
  width: 50%;
  padding: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #d3b872;
  font-size: 2em;
  border-radius: 10px;
  @media (max-width: 940px) {
    height: 175px;
    flex-direction: column;
    width: 100%;
  }
`;

const ValueContainer = styled(Box)`
  height: 100%;
  @media (max-width: 940px) {
    padding: 0;
  }
`;
