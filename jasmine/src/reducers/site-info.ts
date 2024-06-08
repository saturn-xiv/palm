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
  languages: [],
  authors: [],
  version: "",
};

export const layout2state = (res: SiteLayoutResponse): IState => {
  var it: IState = {
    favicon: res.getFavicon(),
    version: res.getVersion(),
    title: res.getTitle(),
    subhead: res.getSubhead(),
    description: res.getDescription(),
    copyright: res.getCopyright(),
    languages: res.getLanguagesList(),
    keywords: res.getKeywordsList(),
    authors: res.getAuthorsList().map((x) => {
      return { name: x.getName(), email: x.getEmail() };
    }),
    icp: res.getIcp()?.getCode(),
  };
  {
    const g = res.getGab();
    if (g) {
      it.gab = { code: g.getCode(), name: g.getName() };
    }
  }
  return it;
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
      state.languages = action.payload.languages;
      state.authors = action.payload.authors;
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
