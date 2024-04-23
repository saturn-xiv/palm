import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { SiteLayoutResponse } from "../protocols/lilac/auth_pb";

interface IGab {
  code: string;
  name: string;
}
interface IAuthor {
  name: string;
  email: string;
}
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
  icp?: string;
  gab?: IGab;
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
    refresh: (state, action: PayloadAction<SiteLayoutResponse>) => {
      state.favicon = action.payload.getFavicon();
      state.version = action.payload.getVersion();
      state.title = action.payload.getTitle();
      state.subhead = action.payload.getSubhead();
      state.description = action.payload.getDescription();
      state.copyright = action.payload.getCopyright();
      state.keywords = action.payload.getKeywordsList();
      state.languages = action.payload.getLanguagesList();
      for (var it of action.payload.getAuthorsList()) {
        state.authors.push({ email: it.getEmail(), name: it.getName() });
      }
      state.icp = action.payload.getIcp()?.getCode();
      {
        const it = action.payload.getGab();
        if (it) {
          state.gab = { code: it.getCode(), name: it.getName() };
        }
      }
    },
  },
});

export const { refresh } = siteInfoSlice.actions;

export const siteInfo = (state: RootState) => state.siteInfo;

export default siteInfoSlice.reducer;
