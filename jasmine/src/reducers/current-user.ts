import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { get as getLocale } from "../locales";
import { guess_timezone } from "../utils";
import { ICurrentUser as IState, ISignInResponse } from "../api/users";

const KEY = "token";
export const DURATION = 60 * 60 * 24;

export const MAX_PASSWORD_LENGTH = 31;
export const MIN_PASSWORD_LENGTH = 6;

const ENABLE_LOCAL_TOKEN = import.meta.env.VITE_ENABLE_LOCAL_TOKEN === "true";

export const get = (): string | null => {
  const token = sessionStorage.getItem(KEY);
  if (token) {
    return token;
  }
  if (ENABLE_LOCAL_TOKEN) {
    return localStorage.getItem(KEY);
  }
  return null;
};

export const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
  if (ENABLE_LOCAL_TOKEN) {
    localStorage.setItem(KEY, token);
  }
};

export const remove = () => {
  sessionStorage.removeItem(KEY);
  localStorage.removeItem(KEY);
};

const initialState: IState = {
  realName: "",
  avatar: "",
  isAdministrator: false,
  isRoot: false,
  roles: [],
  permissions: [],
  hasWechatMiniProgram: false,
  hasWechatOauth2: false,
  hasGoogle: false,
  providerType: "",
  lang: getLocale(),
  timezone: guess_timezone(),
  email: "who-am-i@change-me.org",
  nickname: "nill-gate",
};

interface IProfile {
  realName: string;
  avatar: string;
  lang: string;
  timezone: string;
}

export const currentUserSlice = createSlice({
  name: "current-user",
  initialState,
  reducers: {
    updateProfile: (state, action: PayloadAction<IProfile>) => {
      state.realName = action.payload.realName;
      state.avatar = action.payload.avatar;
      state.lang = action.payload.lang;
      state.timezone = action.payload.timezone;
    },
    refresh: (state, action: PayloadAction<IState>) => {
      state.realName = action.payload.realName;
      state.avatar = action.payload.avatar;
      state.isAdministrator = action.payload.isAdministrator;
      state.isRoot = action.payload.isRoot;
      state.roles = [...action.payload.roles];
      state.permissions = [...action.payload.permissions];
      state.hasWechatMiniProgram = action.payload.hasWechatMiniProgram;
      state.hasWechatOauth2 = action.payload.hasWechatOauth2;
      state.hasGoogle = action.payload.hasGoogle;
      state.providerType = action.payload.providerType;
      state.lang = action.payload.lang;
      state.timezone = action.payload.timezone;
      state.email = action.payload.email;
      state.nickname = action.payload.nickname;
    },
    signIn: (state, action: PayloadAction<ISignInResponse>) => {
      set(action.payload.token);
      state.realName = action.payload.user.realName;
      state.avatar = action.payload.user.avatar;
      state.isAdministrator = action.payload.user.isAdministrator;
      state.isRoot = action.payload.user.isRoot;
      state.roles = [...action.payload.user.roles];
      state.permissions = [...action.payload.user.permissions];
      state.hasWechatMiniProgram = action.payload.user.hasWechatMiniProgram;
      state.hasWechatOauth2 = action.payload.user.hasWechatOauth2;
      state.hasGoogle = action.payload.user.hasGoogle;
      state.providerType = action.payload.user.providerType;
      state.lang = action.payload.user.lang;
      state.timezone = action.payload.user.timezone;
      state.email = action.payload.user.email;
      state.nickname = action.payload.user.nickname;
    },
    signOut: (state) => {
      remove();
      state.realName = "";
      state.avatar = "";
      state.isAdministrator = false;
      state.isRoot = false;
      state.roles = [];
      state.permissions = [];
      state.hasWechatMiniProgram = false;
      state.hasWechatOauth2 = false;
      state.hasGoogle = false;
      state.providerType = "";
      state.lang = getLocale();
      state.timezone = guess_timezone();
    },
  },
});

export const { refresh, updateProfile, signIn, signOut } =
  currentUserSlice.actions;

export const isSignedIn = (state: RootState) =>
  state.currentUser.providerType !== "";
export const currentUser = (state: RootState) => state.currentUser;

export default currentUserSlice.reducer;
