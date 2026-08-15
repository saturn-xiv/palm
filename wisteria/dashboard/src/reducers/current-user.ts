import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { jwtDecode, type JwtPayload } from "jwt-decode";

import type { RootState } from "../store";
import { type ILayout as IUserLayout } from "../api/portal/user";

interface IState {
  payload?: IUserLayout;
}

const initialState: IState = {};

export const slice = createSlice({
  name: "current-user",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<IUserLayout>) => {
      state.payload = structuredClone(action.payload);
    },
    signOut: (state) => {
      state.payload = undefined;
      remove();
    },
    signIn: (
      state,
      action: PayloadAction<{ token: string; user: IUserLayout }>,
    ) => {
      try {
        jwtDecode<JwtPayload>(action.payload.token);
        state.payload = structuredClone(action.payload.user);
        set(action.payload.token);
      } catch (e) {
        console.error(e);
        state.payload = undefined;
        remove();
      }
    },
  },
});

export const { refresh, signIn, signOut } = slice.actions;

export const selectRoles = (state: RootState) =>
  state.currentUser.payload?.roles || [];
export const selectIsAdministrator = (state: RootState) =>
  state.currentUser.payload?.isAdministrator || false;
export const selectPermissions = (state: RootState) =>
  state.currentUser.payload?.permissions || [];
export const selectLayout = (state: RootState) => state.currentUser.payload;

export default slice.reducer;

// --------------------------------------------------------

const KEY = "token";

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

const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
  if (ENABLE_LOCAL_TOKEN) {
    localStorage.setItem(KEY, token);
  }
};

const remove = () => {
  sessionStorage.removeItem(KEY);
  localStorage.removeItem(KEY);
};
