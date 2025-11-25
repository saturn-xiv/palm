import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export interface IPage {
  hostname: string;
  version: string;
  description: string;
}

interface LayoutState {
  version?: string;
  hostname?: string;
  description?: string;
}

const initialState: LayoutState = {};

export const layoutSlice = createSlice({
  name: "layout",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<IPage>) => {
      state.version = action.payload.version;
      state.hostname = action.payload.hostname;
      state.description = action.payload.description;
    },
  },
});

export const { refresh } = layoutSlice.actions;

export const selectVersion = (state: RootState) => state.layout.version;

export default layoutSlice.reducer;
