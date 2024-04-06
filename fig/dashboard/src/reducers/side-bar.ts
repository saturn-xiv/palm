import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

interface IState {
  pathname: string;
}

const initialState: IState = {
  pathname: "",
};

export const sideBarSlice = createSlice({
  name: "side-bar",
  initialState,
  reducers: {
    goto: (state, action: PayloadAction<string>) => {
      state.pathname = action.payload;
    },
  },
});

export const { goto } = sideBarSlice.actions;

export const sideBar = (state: RootState) => state.sideBar;

export default sideBarSlice.reducer;
