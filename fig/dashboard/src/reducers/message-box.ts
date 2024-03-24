import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { AlertColor } from "@mui/material/Alert";

import type { RootState } from "../store";

interface IState {
  severity: AlertColor;
  messages: string[];
}

const initialState: IState = {
  messages: [],
  severity: "info",
};

export const messageBoxSlice = createSlice({
  name: "message-box",
  initialState,
  reducers: {
    success: (state, action: PayloadAction<string[]>) => {
      state.severity = "success";
      state.messages = action.payload;
    },
    info: (state, action: PayloadAction<string[]>) => {
      state.severity = "info";
      state.messages = action.payload;
    },
    error: (state, action: PayloadAction<string[]>) => {
      state.severity = "error";
      state.messages = action.payload;
    },
    warning: (state, action: PayloadAction<string[]>) => {
      state.severity = "warning";
      state.messages = action.payload;
    },
    hide: (state) => {
      state.severity = "info";
      state.messages = [];
    },
  },
});

export const { hide, info, error, warning, success } = messageBoxSlice.actions;

export const messageBox = (state: RootState) => state.messageBox;

export default messageBoxSlice.reducer;
