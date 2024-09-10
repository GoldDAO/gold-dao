import { configureStore } from '@reduxjs/toolkit';

import swapReducer from './swap/swap-slice';
import transferReducer from './transfer/transfer-slice';
import { walletSlice } from '@amerej/artemis-react'

export const store = configureStore({
  reducer: {
    swap:swapReducer,
    transfer:transferReducer,
    wallet: walletSlice.reducer
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({ serializableCheck: false}),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
