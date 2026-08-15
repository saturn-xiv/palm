import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import { type RootState } from "../store";
import { type ILayout as ISiteLayout } from "../api/portal/site";

interface IState {
  payload?: ISiteLayout;
}

const initialState: IState = {};

export const slice = createSlice({
  name: "layout",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<ISiteLayout>) => {
      state.payload = structuredClone(action.payload)
    },
  },
});

export const { refresh } = slice.actions;

export const selectLayout = (state: RootState) => state.layout;

export default slice.reducer;
