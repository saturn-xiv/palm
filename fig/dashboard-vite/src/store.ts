import { configureStore } from "@reduxjs/toolkit";

import siteInfoReducer from "./reducers/site-info";
import currentUserReducer from "./reducers/current-user";
import sideBarReducer from "./reducers/side-bar";

export const store = configureStore({
  reducer: {
    currentUser: currentUserReducer,
    siteInfo: siteInfoReducer,
    sideBar: sideBarReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;
