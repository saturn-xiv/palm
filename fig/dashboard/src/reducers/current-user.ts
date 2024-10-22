import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export const ROLE_ROOT = "root";
export const ROLE_ADMINISTRATOR = "administrator";
export const SESSION_LIFETIME =
  parseInt(import.meta.env.VITE_SESSION_LIFETIME_IN_MINUTES) * 60;

const KEY = "token";

export const get = (): string | null => {
  return sessionStorage.getItem(KEY);
};

const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
};

const remove = () => {
  sessionStorage.removeItem(KEY);
};

export interface IPermission {
  operation: string;
  resource: IResource;
}

export interface IResource {
  type: string;
  id?: number;
}

export interface IUserSignInAction {
  token: string;
}

interface IState {
  uid?: string;
  roles: string[];
  permissions: IPermission[];
}

const initialState: IState = {
  roles: [],
  permissions: [],
};

export const currentUserSlice = createSlice({
  name: "current-user",
  initialState,
  reducers: {
    signIn: (state, action: PayloadAction<IUserSignInAction>) => {
      // TODO
      set(action.payload.token);
      state.uid = "who-am-i";
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

export default currentUserSlice.reducer;
