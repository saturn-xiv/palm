import { configureStore } from "@reduxjs/toolkit";

import siteInfoReducer from "./reducers/site-info";

export const store = configureStore({
  reducer: {
    siteInfo: siteInfoReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;
