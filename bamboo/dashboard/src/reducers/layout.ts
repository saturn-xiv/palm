import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export interface IUpdateLayout {
  version: string;
}

interface LayoutState {
  version?: string;
}

const initialState: LayoutState = {};

export const layoutSlice = createSlice({
  name: "layout",
  initialState,
  reducers: {
    updateLayout: (state, action: PayloadAction<IUpdateLayout>) => {
      state.version = action.payload.version;
    },
  },
});

export const { updateLayout } = layoutSlice.actions;

export const selectVersion = (state: RootState) => state.layout.version;

export default layoutSlice.reducer;
