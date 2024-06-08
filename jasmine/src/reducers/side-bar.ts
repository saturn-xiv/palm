import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";
import { USERS_SIGN_IN_PATH } from "../Router";

interface IState {
  pathname: string;
}

const initialState: IState = {
  pathname: USERS_SIGN_IN_PATH,
};

export const sideBarSlice = createSlice({
  name: "side-bar",
  initialState,
  reducers: {
    set_pathname: (state, action: PayloadAction<string>) => {
      state.pathname = action.payload;
    },
  },
});

export const { set_pathname } = sideBarSlice.actions;

export const sideBar = (state: RootState) => state.sideBar;

export default sideBarSlice.reducer;
