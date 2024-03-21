import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { ISignInResponse } from "../api/camelia";

const KEY = "token";
export const DURATION = 60 * 60 * 24;

export const SIGN_IN_PATH = "/anonymous/users/sign-in";
export const PERSONAL_PATH = "/dashboard/self";

export const MAX_PASSWORD_LENGTH = 31;
export const MIN_PASSWORD_LENGTH = 6;

const ENABLE_LOCAL_TOKEN = import.meta.env.VITE_ENABLE_LOCAL_TOKEN === "true";

export interface IResource {
  type: string;
  iid: number | null;
  sid: string | null;
}
export interface IPermission {
  resource: IResource;
  action: string;
}

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

export interface IState {
  realName: string;
  isAdministrator: boolean;
  isRoot: boolean;
  roles: string[];
  permissions: IPermission[];
  hasWechatMiniProgram: boolean;
  hasWechatOauth2: boolean;
  hasGoogle: boolean;
  providerType: string | null;
}

const initialState: IState = {
  realName: "",
  isAdministrator: false,
  isRoot: false,
  roles: [],
  permissions: [],
  hasWechatMiniProgram: false,
  hasWechatOauth2: false,
  hasGoogle: false,
  providerType: null,
};

export const currentUserSlice = createSlice({
  name: "current-user",
  initialState,
  reducers: {
    signIn: (state, action: PayloadAction<ISignInResponse>) => {
      console.log(action.payload);
      set(action.payload.token);
      state.realName = action.payload.realName;
      state.isAdministrator = action.payload.isAdministrator;
      state.isRoot = action.payload.isRoot;
      state.roles = [...action.payload.roles];
      state.permissions = [...action.payload.permissions];
      state.hasWechatMiniProgram = action.payload.hasWechatMiniProgram;
      state.hasWechatOauth2 = action.payload.hasWechatOauth2;
      state.hasGoogle = action.payload.hasGoogle;
      state.providerType = action.payload.providerType;
    },
    signOut: (state) => {
      remove();
      state.realName = "";
      state.isAdministrator = false;
      state.isRoot = false;
      state.roles = [];
      state.permissions = [];
      state.hasWechatMiniProgram = false;
      state.hasWechatOauth2 = false;
      state.hasGoogle = false;
      state.providerType = null;
    },
  },
});

export const { signIn, signOut } = currentUserSlice.actions;

export const selectIsSignIn = (state: RootState) =>
  state.currentUser.providerType !== null;
export const selectCurrentUser = (state: RootState) => state.currentUser;

export default currentUserSlice.reducer;
