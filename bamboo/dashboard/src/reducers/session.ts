import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import * as jose from "jose";

import type { RootState } from "../store";

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
    },
    signIn: (state, action: PayloadAction<ISignIn>) => {
      const claims = jose.decodeJwt(action.payload.token);
      if (claims.sub) {
        state.name = claims.sub;
      }
    },
  },
});

export const { signIn, signOut } = sessionSlice.actions;

export const selectName = (state: RootState) => state.session.name;

export default sessionSlice.reducer;
