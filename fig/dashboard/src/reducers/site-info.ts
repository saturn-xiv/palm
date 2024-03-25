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
  icpCode?: string;
  gabCode?: string;
}

const initialState: IState = {
  title: "",
  subhead: "",
  description: "",
  keywords: [],
  languages: ["en-US", "zh-Hans"],
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
      state.icpCode = action.payload.siteInfo.icpCode;
      state.gabCode = action.payload.siteInfo.gabCode;
    },
  },
});

export const { refresh } = siteInfoSlice.actions;

export const siteInfo = (state: RootState) => state.siteInfo;

export default siteInfoSlice.reducer;
