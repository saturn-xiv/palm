import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { ILayoutResponse, IAuthor } from "../api/camelia";

interface IState {
  title: string;
  subhead: string;
  description: string;
  keywords: string[];
  languages: string[];
  authors: IAuthor[];
  version: string;
}

const initialState: IState = {
  title: "",
  subhead: "",
  description: "",
  keywords: [],
  languages: [],
  authors: [],
  version: "",
};

export const siteInfoSlice = createSlice({
  name: "site-info",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<ILayoutResponse>) => {
      state.version = action.payload.apiVersion;
      state.title = action.payload.siteInfo.title;
      state.subhead = action.payload.siteInfo.subhead;
      state.description = action.payload.siteInfo.description;
      state.keywords = action.payload.siteInfo.keywords;
      state.languages = action.payload.siteInfo.languages;
      state.authors = action.payload.siteInfo.authors;
    },
  },
});

export const { refresh } = siteInfoSlice.actions;

export const selectSiteInfo = (state: RootState) => state.siteInfo;

export default siteInfoSlice.reducer;
