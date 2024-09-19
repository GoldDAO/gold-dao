import { createSlice, PayloadAction } from '@reduxjs/toolkit';

import type { RootState } from '../index';
import { useAppSelector } from '../hooks'

export enum Token {
  GLDT,
  GLD_NFT,
  OGY,
}

export interface TransferStoreState {
  token: Token;
}

const initialState: TransferStoreState = {
  token: Token.GLDT
};

export const transferSlice = createSlice({
  name: 'transfer',
  initialState,
  reducers: {
    setToken: (state, action:PayloadAction<Token>): void => {
      const token = action.payload
      if (token === Token.GLDT) {
        state.token=(Token.GLDT);
      } else if (token === Token.GLD_NFT) {
        state.token=(Token.GLD_NFT);
      } else {
        state.token=(Token.OGY);
      }
      // searchParams.set("token", token.toString());
      // setSearchParams(searchParams);
    },
    resetTransfer:(state)=>{
      state.token=Token.GLDT;
    },
  },
});

export const transferActions = { ...transferSlice.actions};

const selectTransferState = (state: RootState): TransferStoreState => state.transfer;

export const useTransferStore = (): TransferStoreState =>useAppSelector(selectTransferState);

export default transferSlice.reducer;
