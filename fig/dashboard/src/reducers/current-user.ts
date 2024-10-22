import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { RootState } from "../store";

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
      console.log(action.payload.token);
      state.uid = "who-am-i";
    },
    signOut: (state) => {
      state.uid = undefined;
    },
  },
});

export const { signIn, signOut } = currentUserSlice.actions;

export const isSignIn = (state: RootState) =>
  state.currentUser.uid !== undefined;

export default currentUserSlice.reducer;
