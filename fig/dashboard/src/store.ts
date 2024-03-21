import { configureStore } from "@reduxjs/toolkit";

import siteInfoReducer from "./reducers/site-info";
import currentUserReducer from "./reducers/current-user";

export const store = configureStore({
  reducer: {
    currentUser: currentUserReducer,
    siteInfo: siteInfoReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;
