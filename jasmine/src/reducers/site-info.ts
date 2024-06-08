import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import { ILayout as IState } from "../api/site";
import type { RootState } from "../store";
import { get as get_locale } from "../locales";

const initialState: IState = {
  favicon: "",
  title: "",
  subhead: "",
  description: "",
  copyright: "",
  keywords: [],
  locale: get_locale(),
  languages: [],
  version: "",
};

export const siteInfoSlice = createSlice({
  name: "site-info",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<IState>) => {
      state.favicon = action.payload.favicon;
      state.version = action.payload.version;
      state.title = action.payload.title;
      state.subhead = action.payload.subhead;
      state.description = action.payload.description;
      state.copyright = action.payload.copyright;
      state.keywords = action.payload.keywords;
      state.locale = action.payload.locale;
      state.languages = action.payload.languages;
      {
        const it = action.payload.author;
        if (it) {
          state.author = it;
        }
      }
      state.icp = action.payload.icp;
      {
        const it = action.payload.gab;
        if (it) {
          state.gab = { code: it.code, name: it.name };
        }
      }
    },
  },
});

export const { refresh } = siteInfoSlice.actions;

export const siteInfo = (state: RootState) => state.siteInfo;

export default siteInfoSlice.reducer;
