import { createSlice, PayloadAction } from '@reduxjs/toolkit';

import type { RootState } from '../index';
import { useAppSelector } from '../hooks'

export enum Mode {
  GLD_NFT_TO_GLDT,
  GLDT_TO_GLD_NFT,
}

export interface SwapStoreState {
  mode: Mode;
}

const initialState: SwapStoreState = {
  mode: Mode.GLD_NFT_TO_GLDT
};

export const swapSlice = createSlice({
  name: 'swap',
  initialState,
  reducers: {
    setMode:(state , action:PayloadAction<Mode>): void => {
      if (action.payload === Mode.GLD_NFT_TO_GLDT) {
        state.mode = Mode.GLD_NFT_TO_GLDT
      } else {
        state.mode = Mode.GLDT_TO_GLD_NFT;
      }
    },
    resetSwap:(state)=>{
      state.mode=Mode.GLD_NFT_TO_GLDT;
    },
  },
});

export const swapActions = { ...swapSlice.actions};

const selectSwapState = (state: RootState): SwapStoreState => state.swap;

export const useSwapStore = (): SwapStoreState =>useAppSelector(selectSwapState);

export default swapSlice.reducer;
