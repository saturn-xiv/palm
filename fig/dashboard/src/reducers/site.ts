import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export interface ISiteInfo {
  favicon: string;
  title: string;
  subhead: string;
  author: IAuthor;
  keywords: string[];
  description: string;
  copyright: string;
  cnMps?: { code: string; name: string };
  cnIcp?: { code: string };
  locale: string;
  languages: string[];
}

export interface IAuthor {
  name: string;
  email: string;
}

interface IState {
  info?: ISiteInfo;
}

const initialState: IState = {};

export const siteSlice = createSlice({
  name: "site",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<ISiteInfo>) => {
      state.info = Object.assign({}, action.payload);
    },
  },
});

export const { refresh } = siteSlice.actions;

export const siteInfo = (state: RootState) => state.site.info;

export default siteSlice.reducer;
