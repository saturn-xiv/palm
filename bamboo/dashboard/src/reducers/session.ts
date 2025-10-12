import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import jwt from "jsonwebtoken";

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
      const decoded = jwt.decode(action.payload.token, { complete: true });
      const name = decoded?.payload.sub;
      if (typeof name === "string") {
        state.name = name;
      } else {
        state.name = undefined;
      }
    },
  },
});

export const { signIn, signOut } = sessionSlice.actions;

export const selectName = (state: RootState) => state.session.name;

export default sessionSlice.reducer;
