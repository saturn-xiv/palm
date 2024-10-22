import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export interface ISiteLayout {
  title: string;
  subhead: string;
  copyright: string;
}

interface IState {
  layout?: ISiteLayout;
}

const initialState: IState = {};

export const siteSlice = createSlice({
  name: "site",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<ISiteLayout>) => {
      state.layout = {
        title: action.payload.title,
        subhead: action.payload.subhead,
        copyright: action.payload.copyright,
      };
    },
  },
});

export const { refresh } = siteSlice.actions;

export const siteLayout = (state: RootState) => state.site.layout;

export default siteSlice.reducer;
