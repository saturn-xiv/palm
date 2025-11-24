import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import * as jose from "jose";

import type { RootState } from "../store";

export const SIGN_IN = "/anonymous/sign-in";
export const LOGS = "/dashboard/account/logs";

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

export interface ISignIn {
  token: string;
}

interface SessionState {
  name?: string;
}

const initialState: SessionState = {};

export const sessionSlice = createSlice({
  name: "session",
  initialState,
  reducers: {
    signOut: (state) => {
      state.name = undefined;
      remove();
    },
    signIn: (state, action: PayloadAction<ISignIn>) => {
      try {
        const claims = jose.decodeJwt(action.payload.token);
        if (claims.sub) {
          state.name = claims.sub;
        }
        set(action.payload.token);
      } catch (e) {
        console.error(e);
        state.name = undefined;
      }
    },
  },
});

export const { signIn, signOut } = sessionSlice.actions;

export const currentUser = (state: RootState) => state.session.name;

export default sessionSlice.reducer;
