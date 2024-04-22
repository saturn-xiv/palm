import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { ILayoutResponse, IGabCode, IAuthor } from "../api/camelia";

interface IState {
  favicon: string;
  title: string;
  subhead: string;
  description: string;
  keywords: string[];
  copyright: string;
  languages: string[];
  authors: IAuthor[];
  version: string;
  icpCode?: string;
  gabCode?: IGabCode;
}

const initialState: IState = {
  favicon: "",
  title: "",
  subhead: "",
  description: "",
  copyright: "",
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
      state.favicon = action.payload.siteInfo.favicon;
      state.version = action.payload.apiVersion;
      state.title = action.payload.siteInfo.title;
      state.subhead = action.payload.siteInfo.subhead;
      state.description = action.payload.siteInfo.description;
      state.copyright = action.payload.siteInfo.copyright;
      state.keywords = action.payload.siteInfo.keywords;
      state.languages = action.payload.siteInfo.languages;
      state.authors = action.payload.siteInfo.authors;
      state.icpCode = action.payload.siteInfo.icpCode?.code;
      state.gabCode = action.payload.siteInfo.gabCode;
    },
  },
});

export const { refresh } = siteInfoSlice.actions;

export const siteInfo = (state: RootState) => state.siteInfo;

export default siteInfoSlice.reducer;
