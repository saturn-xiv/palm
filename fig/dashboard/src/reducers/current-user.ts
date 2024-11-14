import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { jwtDecode, JwtPayload } from "jwt-decode";

import type { RootState } from "../store";

export const SIGN_IN_PATH = "/anonymous/users/sign-in";
export const PERSONAL_PATH = "/dashboard/personal";

export const ROLE_ROOT = "root";
export const ROLE_ADMINISTRATOR = "administrator";
export const SESSION_LIFETIME =
  parseInt(import.meta.env.VITE_SESSION_LIFETIME_IN_MINUTES) * 60;

const KEY = "token.v20241013";

export const get = (): string | null => {
  return sessionStorage.getItem(KEY);
};

const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
};

const remove = () => {
  sessionStorage.removeItem(KEY);
};

export interface IResource {
  type: string;
  id?: number;
}
export interface IPermission {
  operation: string;
  resource: IResource;
}
export interface IMenu {
  label: string;
  to: string;
  external: boolean;
  children?: IMenu[];
}
export enum IProviderType {
  EMAIL,
  GOOGLE,
  FACEBOOK,
  WECHAT_MINI_PROGRAM,
  WECHAT_OAUTH2,
}

export interface ICurrentUser {
  realName: string;
  providerType: IProviderType;
  lang: string;
  timezone: string;
  isAdministrator: boolean;
  isRoot: boolean;
  roles: string[];
  permissions: IPermission[];
  sideBar: IMenu[];
}

export interface IState {
  uid?: string;
  profile?: ICurrentUser;
}

const initialState: IState = {};

export const currentUserSlice = createSlice({
  name: "current-user",
  initialState,
  reducers: {
    signIn: (
      state,
      action: PayloadAction<{ token: string; profile: ICurrentUser }>
    ) => {
      try {
        const decoded = jwtDecode<JwtPayload>(action.payload.token);
        if (decoded.sub) {
          set(action.payload.token);
          state.uid = decoded.sub;
          state.profile = Object.assign({}, action.payload.profile);
        }
        return;
      } catch (e) {
        console.error(e);
      }
      state.uid = undefined;
      state.profile = undefined;
      remove();
    },
    signOut: (state) => {
      remove();
      state.uid = undefined;
    },
  },
});

export const { signIn, signOut } = currentUserSlice.actions;

export const isSignIn = (state: RootState) =>
  state.currentUser.uid !== undefined;
export const currentUser = (state: RootState) => state.currentUser.profile;

export default currentUserSlice.reducer;
