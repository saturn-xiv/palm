import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

export interface ISignIn {
  name: string;
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
      state.name = action.payload.name;
    },
  },
});

export const { signIn, signOut } = sessionSlice.actions;

export const selectName = (state: RootState) => state.session.name;

export default sessionSlice.reducer;
