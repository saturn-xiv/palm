import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import { type RootState } from "../store";
import { type ILayout as IUserLayout } from "../api/portal/user";

interface IMenu {
  code: string;
  children?: IMenu[];
}

interface IState {
  items: IMenu[];
}

const initialState: IState = {
  items: [],
};

export const slice = createSlice({
  name: "side-bar",
  initialState,
  reducers: {
    refresh: (state, action: PayloadAction<IUserLayout>) => {
      const items: IMenu[] = [];
      if (action.payload.isAdministrator) {
        items.push({ code: "site" });
      }

      state.items = items;
    },
  },
});

export const { refresh } = slice.actions;

export const selectMenus = (state: RootState) => state.sideBar.items;

export default slice.reducer;
