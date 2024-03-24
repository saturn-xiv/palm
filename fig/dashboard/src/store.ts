import { configureStore } from "@reduxjs/toolkit";

import siteInfoReducer from "./reducers/site-info";
import currentUserReducer from "./reducers/current-user";
import messageBoxReducer from "./reducers/message-box";

export const store = configureStore({
  reducer: {
    currentUser: currentUserReducer,
    siteInfo: siteInfoReducer,
    messageBox: messageBoxReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;
